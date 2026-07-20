export type Nullable<T> = T | null;

export type AvailabilityStatus = 'accepting' | 'not_accepting' | 'unknown';

export type AvailabilityReportKind = 'confirm_accepting' | 'status_change';

export interface ApiErrorBody {
  error?: {
    code?: string;
    message?: string;
  };
}

export interface CitySearchResult {
  kind: 'city' | 'area';
  label: string;
  cityId: number;
  cityName: string;
  citySlug: string;
  provinceCode: string;
  areaId: Nullable<number>;
  areaName: Nullable<string>;
  areaSlug: Nullable<string>;
  latitude: Nullable<number>;
  longitude: Nullable<number>;
}

export interface CitySearchResponse {
  results: CitySearchResult[];
}

export interface ListingCity {
  id: number;
  name: string;
  slug: string;
  provinceCode: string;
  latitude: Nullable<number>;
  longitude: Nullable<number>;
}

export interface ListingArea {
  id: number;
  name: string;
  slug: string;
}

export interface ListingClinic {
  id: number;
  name: string;
  slug: string;
  addressLine1: string;
  addressLine2: Nullable<string>;
  municipality: string;
  provinceCode: string;
  postalCode: Nullable<string>;
  phone: Nullable<string>;
  fax: Nullable<string>;
  email: Nullable<string>;
  websiteUrl: Nullable<string>;
  latitude: Nullable<number>;
  longitude: Nullable<number>;
}

export interface ListingStatus {
  family_doctor_id: number;
  current_status: AvailabilityStatus;
  last_reported_at: Nullable<string>;
  last_confirmed_accepting_at: Nullable<string>;
  last_confirmed_accepting_days_ago: Nullable<number>;
}

export interface DoctorListing {
  id: number;
  slug: string;
  fullName: string;
  credentials: Nullable<string>;
  phone: Nullable<string>;
  email: Nullable<string>;
  profileUrl: Nullable<string>;
  clinic: ListingClinic;
  area: ListingArea;
  status: ListingStatus;
}

export interface CityDoctorListings {
  city: ListingCity;
  listings: DoctorListing[];
}

export interface ReportHistoryItem {
  id: number;
  reportKind: AvailabilityReportKind;
  reportedStatus: AvailabilityStatus;
  note: Nullable<string>;
  submittedAt: string;
}

export interface DoctorDetail extends DoctorListing {
  city: ListingCity;
  reportHistory: ReportHistoryItem[];
}

export interface ReportSubmissionResponse {
  doctorId: number;
  report: ReportHistoryItem;
  status: ListingStatus;
}

export interface CitySearchParams {
  query?: string;
  limit?: number;
}

export interface DoctorListingsParams {
  citySlug: string;
  areaSlug?: string;
  limit?: number;
}

export interface ReportSubmissionInput {
  note?: string;
}
