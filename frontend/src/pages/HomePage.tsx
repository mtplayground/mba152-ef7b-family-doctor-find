import { useSearchParams } from 'react-router-dom';
import { ApiError } from '../api/client';
import { useDoctorListings } from '../api/hooks';
import type { CitySearchResult, DoctorListing } from '../api/types';
import { CityTypeahead } from '../components/CityTypeahead';

export function HomePage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const selectedCitySlug = searchParams.get('city') ?? '';
  const selectedAreaSlug = searchParams.get('area') ?? undefined;
  const selectedLabel = searchParams.get('label') ?? '';
  const listings = useDoctorListings({
    citySlug: selectedCitySlug,
    areaSlug: selectedAreaSlug,
    limit: 12,
  });

  function handleSelect(result: CitySearchResult) {
    const params = new URLSearchParams();
    params.set('city', result.citySlug);
    params.set('label', result.label);

    if (result.areaSlug) {
      params.set('area', result.areaSlug);
    }

    setSearchParams(params);
  }

  return (
    <div className="grid gap-8">
      <section className="grid gap-6 rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8 lg:grid-cols-[minmax(0,1fr)_420px] lg:items-end">
        <div>
          <p className="mb-3 text-sm font-semibold uppercase text-civic-700">
            Public-service directory
          </p>
          <h1 className="max-w-3xl text-4xl font-semibold leading-tight text-ink-900 sm:text-5xl">
            Find family doctor availability by Canadian city.
          </h1>
          <p className="mt-5 max-w-2xl text-lg text-ink-700">
            Search a city or area to scan clinics, contact details, and recent
            community availability reports.
          </p>
        </div>

        <CityTypeahead initialValue={selectedLabel} onSelect={handleSelect} />
      </section>

      <ResultsPanel
        selectedLabel={selectedLabel}
        selectedCitySlug={selectedCitySlug}
        listings={listings.data?.listings ?? []}
        isLoading={listings.isPending && selectedCitySlug.length > 0}
        error={listings.error}
      />
    </div>
  );
}

interface ResultsPanelProps {
  selectedLabel: string;
  selectedCitySlug: string;
  listings: DoctorListing[];
  isLoading: boolean;
  error: Error | null;
}

function ResultsPanel({
  selectedLabel,
  selectedCitySlug,
  listings,
  isLoading,
  error,
}: ResultsPanelProps) {
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
          Loading {selectedLabel}
        </h2>
        <div className="mt-6 grid gap-3">
          {[0, 1, 2].map((item) => (
            <div
              key={item}
              className="h-24 animate-pulse rounded-lg bg-surface-muted"
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
    <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
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
        <div className="mt-6 grid gap-4">
          {listings.map((listing) => (
            <DoctorListingCard key={listing.id} listing={listing} />
          ))}
        </div>
      )}
    </section>
  );
}

function DoctorListingCard({ listing }: { listing: DoctorListing }) {
  return (
    <article className="rounded-lg border border-ink-100 bg-white p-5">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h3 className="text-lg font-semibold text-ink-900">
            {listing.fullName}
          </h3>
          <p className="mt-1 text-sm text-ink-700">
            {listing.clinic.name} - {listing.area.name}
          </p>
          <p className="mt-2 text-sm text-ink-500">
            {listing.clinic.addressLine1}, {listing.clinic.municipality},{' '}
            {listing.clinic.provinceCode}
          </p>
        </div>
        <span
          className={[
            'inline-flex w-fit rounded-control px-3 py-1 text-xs font-semibold uppercase',
            statusClassName(listing.status.current_status),
          ].join(' ')}
        >
          {statusLabel(listing.status.current_status)}
        </span>
      </div>

      <div className="mt-4 flex flex-wrap gap-x-5 gap-y-2 text-sm text-ink-700">
        {listing.phone ? <span>{listing.phone}</span> : null}
        {listing.email ? <span>{listing.email}</span> : null}
        <span>{recencyLabel(listing)}</span>
      </div>
    </article>
  );
}

function statusLabel(status: DoctorListing['status']['current_status']) {
  if (status === 'accepting') {
    return 'Accepting';
  }

  if (status === 'not_accepting') {
    return 'Not accepting';
  }

  return 'Unknown';
}

function statusClassName(status: DoctorListing['status']['current_status']) {
  if (status === 'accepting') {
    return 'bg-civic-50 text-civic-700';
  }

  if (status === 'not_accepting') {
    return 'bg-service-red/10 text-service-red';
  }

  return 'bg-surface-muted text-ink-700';
}

function recencyLabel(listing: DoctorListing) {
  const daysAgo = listing.status.last_confirmed_accepting_days_ago;

  if (daysAgo === null) {
    return 'No recent accepting confirmation';
  }

  if (daysAgo === 0) {
    return 'Confirmed accepting today';
  }

  if (daysAgo === 1) {
    return 'Confirmed accepting 1 day ago';
  }

  return `Confirmed accepting ${daysAgo} days ago`;
}
