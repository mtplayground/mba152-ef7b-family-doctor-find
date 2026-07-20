import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { directoryApi } from './client';
import type {
  CitySearchParams,
  DoctorDetail,
  DoctorListingsParams,
  ReportSubmissionInput,
  ReportSubmissionResponse,
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
    onSuccess: (response) => {
      updateDoctorDetailCache(queryClient, response);
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
    onSuccess: (response) => {
      updateDoctorDetailCache(queryClient, response);
      void queryClient.invalidateQueries({
        queryKey: directoryQueryKeys.doctorListingsRoot,
      });
    },
  });
}

function updateDoctorDetailCache(
  queryClient: ReturnType<typeof useQueryClient>,
  response: ReportSubmissionResponse,
) {
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
