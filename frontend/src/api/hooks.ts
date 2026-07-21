import {
  useMutation,
  useQuery,
  useQueryClient,
  type QueryClient,
} from '@tanstack/react-query';
import { directoryApi } from './client';
import type {
  AvailabilityReportKind,
  AvailabilityStatus,
  CityDoctorListings,
  CitySearchParams,
  DoctorDetail,
  DoctorListingsParams,
  ReportSubmissionInput,
  ReportSubmissionResponse,
  ListingStatus,
} from './types';

export const directoryQueryKeys = {
  citySearch: (params: CitySearchParams) =>
    ['citySearch', params.query ?? '', params.limit ?? null] as const,
  doctorListingsRoot: ['doctorListings'] as const,
  doctorListings: (params: DoctorListingsParams) =>
    [
      ...directoryQueryKeys.doctorListingsRoot,
      params.citySlug,
      params.areaSlug ?? null,
      params.limit ?? null,
    ] as const,
  doctorDetail: (doctorId: number) => ['doctorDetail', doctorId] as const,
};

export function useCitySearch(params: CitySearchParams) {
  const query = params.query?.trim() ?? '';

  return useQuery({
    queryKey: directoryQueryKeys.citySearch({ ...params, query }),
    queryFn: () => directoryApi.searchCities({ ...params, query }),
    enabled: query.length > 0,
  });
}

export function useDoctorListings(params: DoctorListingsParams) {
  return useQuery({
    queryKey: directoryQueryKeys.doctorListings(params),
    queryFn: () => directoryApi.listDoctorsByCity(params),
    enabled: params.citySlug.trim().length > 0,
  });
}

export function useDoctorDetail(doctorId: number | undefined) {
  return useQuery({
    queryKey: directoryQueryKeys.doctorDetail(doctorId ?? 0),
    queryFn: () => directoryApi.getDoctorDetail(doctorId ?? 0),
    enabled: typeof doctorId === 'number' && doctorId > 0,
  });
}

export function useConfirmAcceptingMutation(doctorId: number) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: ReportSubmissionInput = {}) =>
      directoryApi.confirmAccepting(doctorId, input),
    onMutate: () =>
      applyOptimisticReport(queryClient, doctorId, 'confirm_accepting'),
    onError: (_error, _variables, context) => {
      rollbackOptimisticReport(queryClient, context);
    },
    onSuccess: (response) => {
      updateDoctorCachesFromResponse(queryClient, response);
    },
    onSettled: () => {
      void queryClient.invalidateQueries({
        queryKey: directoryQueryKeys.doctorListingsRoot,
      });
    },
  });
}

export function useStatusChangeMutation(doctorId: number) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: ReportSubmissionInput = {}) =>
      directoryApi.reportStatusChange(doctorId, input),
    onMutate: () =>
      applyOptimisticReport(queryClient, doctorId, 'status_change'),
    onError: (_error, _variables, context) => {
      rollbackOptimisticReport(queryClient, context);
    },
    onSuccess: (response) => {
      updateDoctorCachesFromResponse(queryClient, response);
    },
    onSettled: () => {
      void queryClient.invalidateQueries({
        queryKey: directoryQueryKeys.doctorListingsRoot,
      });
    },
  });
}

interface OptimisticReportContext {
  doctorDetail: DoctorDetail | undefined;
  doctorListings: Array<
    readonly [readonly unknown[], CityDoctorListings | undefined]
  >;
}

async function applyOptimisticReport(
  queryClient: QueryClient,
  doctorId: number,
  reportKind: AvailabilityReportKind,
): Promise<OptimisticReportContext> {
  const now = new Date().toISOString();
  const detailKey = directoryQueryKeys.doctorDetail(doctorId);

  await Promise.all([
    queryClient.cancelQueries({
      queryKey: directoryQueryKeys.doctorListingsRoot,
    }),
    queryClient.cancelQueries({ queryKey: detailKey }),
  ]);

  const context: OptimisticReportContext = {
    doctorDetail: queryClient.getQueryData<DoctorDetail>(detailKey),
    doctorListings: queryClient.getQueriesData<CityDoctorListings>({
      queryKey: directoryQueryKeys.doctorListingsRoot,
    }),
  };

  updateDoctorListingsCache(queryClient, doctorId, (previous) =>
    optimisticStatus(doctorId, reportKind, now, previous),
  );

  queryClient.setQueryData<DoctorDetail>(detailKey, (current) => {
    if (!current) {
      return current;
    }

    return {
      ...current,
      status: optimisticStatus(doctorId, reportKind, now, current.status),
    };
  });

  return context;
}

function rollbackOptimisticReport(
  queryClient: QueryClient,
  context: OptimisticReportContext | undefined,
) {
  if (!context) {
    return;
  }

  for (const [queryKey, data] of context.doctorListings) {
    queryClient.setQueryData(queryKey, data);
  }

  if (context.doctorDetail) {
    queryClient.setQueryData(
      directoryQueryKeys.doctorDetail(context.doctorDetail.id),
      context.doctorDetail,
    );
  }
}

function updateDoctorCachesFromResponse(
  queryClient: QueryClient,
  response: ReportSubmissionResponse,
) {
  updateDoctorListingsCache(
    queryClient,
    response.doctorId,
    () => response.status,
  );

  queryClient.setQueryData<DoctorDetail>(
    directoryQueryKeys.doctorDetail(response.doctorId),
    (current) => {
      if (!current) {
        return current;
      }

      return {
        ...current,
        status: response.status,
        reportHistory: [response.report, ...current.reportHistory],
      };
    },
  );
}

function updateDoctorListingsCache(
  queryClient: QueryClient,
  doctorId: number,
  statusForListing: (previous: ListingStatus) => ListingStatus,
) {
  queryClient.setQueriesData<CityDoctorListings>(
    { queryKey: directoryQueryKeys.doctorListingsRoot },
    (current) => {
      if (!current) {
        return current;
      }

      return {
        ...current,
        listings: current.listings.map((listing) =>
          listing.id === doctorId
            ? {
                ...listing,
                status: statusForListing(listing.status),
              }
            : listing,
        ),
      };
    },
  );
}

function optimisticStatus(
  doctorId: number,
  reportKind: AvailabilityReportKind,
  submittedAt: string,
  previous: ListingStatus,
): ListingStatus {
  const currentStatus = reportStatus(reportKind);

  return {
    ...previous,
    family_doctor_id: previous.family_doctor_id || doctorId,
    current_status: currentStatus,
    last_reported_at: submittedAt,
    last_confirmed_accepting_at:
      reportKind === 'confirm_accepting'
        ? submittedAt
        : previous.last_confirmed_accepting_at,
    last_confirmed_accepting_days_ago:
      reportKind === 'confirm_accepting'
        ? 0
        : previous.last_confirmed_accepting_days_ago,
  };
}

function reportStatus(reportKind: AvailabilityReportKind): AvailabilityStatus {
  return reportKind === 'confirm_accepting' ? 'accepting' : 'not_accepting';
}
