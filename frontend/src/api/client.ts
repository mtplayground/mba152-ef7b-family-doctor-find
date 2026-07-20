import type {
  ApiErrorBody,
  CityDoctorListings,
  CitySearchParams,
  CitySearchResponse,
  DoctorDetail,
  DoctorListingsParams,
  ReportSubmissionInput,
  ReportSubmissionResponse,
} from './types';

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL ?? '').replace(/\/+$/, '');

export class ApiError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.code = code;
  }
}

export const directoryApi = {
  searchCities(params: CitySearchParams = {}) {
    const query = new URLSearchParams();
    addParam(query, 'q', params.query);
    addParam(query, 'limit', params.limit);

    return requestJson<CitySearchResponse>(`/api/cities/search?${query}`);
  },

  listDoctorsByCity(params: DoctorListingsParams) {
    const query = new URLSearchParams();
    addParam(query, 'area', params.areaSlug);
    addParam(query, 'limit', params.limit);

    return requestJson<CityDoctorListings>(
      `/api/cities/${encodeURIComponent(params.citySlug)}/doctors?${query}`,
    );
  },

  getDoctorDetail(doctorId: number) {
    return requestJson<DoctorDetail>(
      `/api/doctors/${encodeURIComponent(String(doctorId))}`,
    );
  },

  confirmAccepting(doctorId: number, input: ReportSubmissionInput = {}) {
    return submitReport(
      `/api/doctors/${encodeURIComponent(String(doctorId))}/confirm-accepting`,
      input,
    );
  },

  reportStatusChange(doctorId: number, input: ReportSubmissionInput = {}) {
    return submitReport(
      `/api/doctors/${encodeURIComponent(String(doctorId))}/status-change`,
      input,
    );
  },
};

function submitReport(path: string, input: ReportSubmissionInput) {
  return requestJson<ReportSubmissionResponse>(path, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      ...(input.note ? { note: input.note } : {}),
    }),
  });
}

function addParam(
  query: URLSearchParams,
  name: string,
  value: string | number | undefined,
) {
  if (value !== undefined && value !== '') {
    query.set(name, String(value));
  }
}

async function requestJson<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${apiBaseUrl}${path}`, {
    credentials: 'same-origin',
    ...init,
  });
  const payload = await parseJson(response);

  if (!response.ok) {
    const errorPayload = payload as ApiErrorBody;
    const code = errorPayload.error?.code ?? 'request_failed';
    const message = errorPayload.error?.message ?? response.statusText;
    throw new ApiError(response.status, code, message);
  }

  return payload as T;
}

async function parseJson(response: Response): Promise<unknown> {
  const text = await response.text();

  if (!text) {
    return null;
  }

  try {
    return JSON.parse(text) as unknown;
  } catch (error) {
    throw new ApiError(
      response.status,
      'invalid_json',
      error instanceof Error ? error.message : 'Invalid JSON response',
    );
  }
}
