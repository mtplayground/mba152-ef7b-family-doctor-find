import { Link } from 'react-router-dom';
import { ApiError } from '../api/client';
import type { AvailabilityStatus, DoctorListing } from '../api/types';
import { ReportControls } from './ReportControls';

interface DoctorResultsListProps {
  listings: DoctorListing[];
  isLoading: boolean;
  error: Error | null;
  selectedLabel: string;
  selectedCitySlug: string;
}

export function DoctorResultsList({
  listings,
  isLoading,
  error,
  selectedLabel,
  selectedCitySlug,
}: DoctorResultsListProps) {
  if (!selectedCitySlug) {
    return (
      <section className="rounded-lg border border-dashed border-civic-100 bg-civic-50 p-6 sm:p-8">
        <h2 className="text-xl font-semibold text-civic-900">
          Start with a city
        </h2>
        <p className="mt-3 max-w-2xl text-sm leading-6 text-ink-700">
          Search suggestions include launch cities and neighbourhood areas. Pick
          one to load matching family doctor listings.
        </p>
      </section>
    );
  }

  if (isLoading) {
    return (
      <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
        <h2 className="text-xl font-semibold text-ink-900">
          Loading {selectedLabel || selectedCitySlug}
        </h2>
        <div className="mt-6 grid gap-3">
          {[0, 1, 2].map((item) => (
            <div
              key={item}
              className="h-28 animate-pulse rounded-lg bg-surface-muted"
            />
          ))}
        </div>
      </section>
    );
  }

  if (error) {
    return (
      <section className="rounded-lg border border-service-red/20 bg-white p-6 shadow-sm sm:p-8">
        <h2 className="text-xl font-semibold text-service-red">
          Results unavailable
        </h2>
        <p className="mt-3 text-sm leading-6 text-ink-700">
          {error instanceof ApiError
            ? error.message
            : 'The directory could not load listings for this search.'}
        </p>
      </section>
    );
  }

  return (
    <section className="rounded-lg border border-ink-100 bg-surface-raised p-4 shadow-sm sm:p-6">
      <div className="flex flex-col gap-2 px-1 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-sm font-semibold uppercase text-civic-700">
            Results
          </p>
          <h2 className="mt-2 text-2xl font-semibold text-ink-900">
            {selectedLabel || selectedCitySlug}
          </h2>
        </div>
        <p className="text-sm text-ink-500">
          {listings.length} {listings.length === 1 ? 'listing' : 'listings'}
        </p>
      </div>

      {listings.length === 0 ? (
        <p className="mt-6 rounded-lg bg-surface-muted px-4 py-3 text-sm text-ink-700">
          No doctor listings are available for this search yet.
        </p>
      ) : (
        <div className="mt-5 divide-y divide-ink-100 overflow-hidden rounded-lg border border-ink-100 bg-white">
          {listings.map((listing) => (
            <DoctorResultRow key={listing.id} listing={listing} />
          ))}
        </div>
      )}
    </section>
  );
}

function DoctorResultRow({ listing }: { listing: DoctorListing }) {
  return (
    <article className="grid gap-4 p-4 transition hover:bg-surface-muted sm:grid-cols-[minmax(0,1fr)_190px] sm:p-5">
      <div className="min-w-0">
        <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
          <div className="min-w-0">
            <h3 className="text-lg font-semibold leading-snug text-ink-900">
              <Link
                to={`/doctors/${listing.id}`}
                className="transition hover:text-civic-700"
              >
                {listing.fullName}
              </Link>
              {listing.credentials ? (
                <span className="ml-2 text-sm font-medium text-ink-500">
                  {listing.credentials}
                </span>
              ) : null}
            </h3>
            <p className="mt-2 text-sm font-semibold text-ink-800">
              {listing.clinic.name}
            </p>
          </div>
          <StatusBadge status={listing.status.current_status} />
        </div>

        <dl className="mt-4 grid gap-3 text-sm sm:grid-cols-3">
          <div>
            <dt className="font-semibold text-ink-500">Area</dt>
            <dd className="mt-1 text-ink-800">{listing.area.name}</dd>
          </div>
          <div className="sm:col-span-2">
            <dt className="font-semibold text-ink-500">Clinic address</dt>
            <dd className="mt-1 text-ink-800">{formatAddress(listing)}</dd>
          </div>
        </dl>

        <div className="mt-4 flex flex-wrap gap-x-5 gap-y-2 text-sm text-ink-700">
          {listing.phone ? <span>{listing.phone}</span> : null}
          {listing.email ? <span>{listing.email}</span> : null}
          {listing.profileUrl ? (
            <a
              href={listing.profileUrl}
              className="font-semibold text-civic-700 hover:text-civic-900"
            >
              Profile
            </a>
          ) : null}
          <Link
            to={`/doctors/${listing.id}`}
            className="font-semibold text-civic-700 hover:text-civic-900"
          >
            Details
          </Link>
        </div>
        <div className="mt-4">
          <ReportControls doctorId={listing.id} compact />
        </div>
      </div>

      <RecencyIndicator listing={listing} />
    </article>
  );
}

function StatusBadge({ status }: { status: AvailabilityStatus }) {
  return (
    <span
      className={[
        'inline-flex w-fit rounded-control px-3 py-1 text-xs font-semibold uppercase',
        statusClassName(status),
      ].join(' ')}
    >
      {statusLabel(status)}
    </span>
  );
}

function RecencyIndicator({ listing }: { listing: DoctorListing }) {
  const recency = recencyCopy(listing);

  return (
    <div
      className={[
        'flex min-h-24 flex-col justify-center rounded-lg border px-4 py-3',
        recency.className,
      ].join(' ')}
    >
      <span className="text-xs font-semibold uppercase">Recency</span>
      <span className="mt-2 text-lg font-semibold leading-snug">
        {recency.primary}
      </span>
      <span className="mt-1 text-xs leading-5">{recency.secondary}</span>
    </div>
  );
}

function recencyCopy(listing: DoctorListing) {
  const daysAgo = listing.status.last_confirmed_accepting_days_ago;
  const lastConfirmed = listing.status.last_confirmed_accepting_at;

  if (daysAgo === null) {
    return {
      primary: 'No confirmation',
      secondary: 'No accepting report has been recorded yet.',
      className: 'border-ink-100 bg-surface-muted text-ink-700',
    };
  }

  const secondary = lastConfirmed
    ? `Last accepting report: ${formatDate(lastConfirmed)}`
    : 'Recent accepting report recorded.';

  if (daysAgo === 0) {
    return {
      primary: 'Confirmed today',
      secondary,
      className: 'border-civic-100 bg-civic-50 text-civic-900',
    };
  }

  if (daysAgo === 1) {
    return {
      primary: 'Confirmed 1 day ago',
      secondary,
      className: 'border-civic-100 bg-civic-50 text-civic-900',
    };
  }

  return {
    primary: `Confirmed ${daysAgo} days ago`,
    secondary,
    className:
      daysAgo <= 14
        ? 'border-civic-100 bg-civic-50 text-civic-900'
        : 'border-ink-100 bg-surface-muted text-ink-700',
  };
}

function formatAddress(listing: DoctorListing) {
  return [
    listing.clinic.addressLine1,
    listing.clinic.addressLine2,
    listing.clinic.municipality,
    listing.clinic.provinceCode,
    listing.clinic.postalCode,
  ]
    .filter(Boolean)
    .join(', ');
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat('en-CA', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  }).format(date);
}

function statusLabel(status: AvailabilityStatus) {
  if (status === 'accepting') {
    return 'Accepting';
  }

  if (status === 'not_accepting') {
    return 'Not accepting';
  }

  return 'Unknown';
}

function statusClassName(status: AvailabilityStatus) {
  if (status === 'accepting') {
    return 'bg-civic-50 text-civic-700';
  }

  if (status === 'not_accepting') {
    return 'bg-service-red/10 text-service-red';
  }

  return 'bg-surface-muted text-ink-700';
}
