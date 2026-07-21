import { useSearchParams } from 'react-router-dom';
import { useDoctorListings } from '../api/hooks';
import type { CitySearchResult } from '../api/types';
import { CityTypeahead } from '../components/CityTypeahead';
import { DoctorResultsList } from '../components/DoctorResultsList';
import { DoctorResultsMap } from '../components/DoctorResultsMap';
import {
  ResultsViewToggle,
  type ResultsViewMode,
} from '../components/ResultsViewToggle';

export function ResultsPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const selectedCitySlug = searchParams.get('city') ?? '';
  const selectedAreaSlug = searchParams.get('area') ?? undefined;
  const selectedLabel = searchParams.get('label') ?? '';
  const viewMode = parseViewMode(searchParams.get('view'));
  const listings = useDoctorListings({
    citySlug: selectedCitySlug,
    areaSlug: selectedAreaSlug,
    limit: 24,
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

  function handleViewChange(nextViewMode: ResultsViewMode) {
    const params = new URLSearchParams(searchParams);
    params.set('view', nextViewMode);
    setSearchParams(params);
  }

  return (
    <div className="grid gap-6">
      <section className="rounded-lg border border-ink-100 bg-surface-raised p-5 shadow-sm sm:p-6">
        <div className="grid gap-5 lg:grid-cols-[minmax(0,1fr)_420px] lg:items-end">
          <div>
            <p className="text-sm font-semibold uppercase text-civic-700">
              Doctor listings
            </p>
            <h1 className="mt-2 text-3xl font-semibold leading-tight text-ink-900 sm:text-4xl">
              {selectedLabel || 'Search results'}
            </h1>
            <p className="mt-3 max-w-2xl text-sm leading-6 text-ink-700">
              Compare doctors by clinic, area, contact details, and the latest
              accepting-status confirmation.
            </p>
          </div>

          <CityTypeahead initialValue={selectedLabel} onSelect={handleSelect} />
        </div>
      </section>

      <div className="flex justify-end">
        <ResultsViewToggle value={viewMode} onChange={handleViewChange} />
      </div>

      {viewMode === 'map' ? (
        <DoctorResultsMap
          selectedLabel={selectedLabel}
          selectedCitySlug={selectedCitySlug}
          city={listings.data?.city}
          listings={listings.data?.listings ?? []}
          isLoading={listings.isPending && selectedCitySlug.length > 0}
          error={listings.error}
          onRetry={() => void listings.refetch()}
        />
      ) : (
        <DoctorResultsList
          selectedLabel={selectedLabel}
          selectedCitySlug={selectedCitySlug}
          listings={listings.data?.listings ?? []}
          isLoading={listings.isPending && selectedCitySlug.length > 0}
          error={listings.error}
          onRetry={() => void listings.refetch()}
        />
      )}
    </div>
  );
}

function parseViewMode(value: string | null): ResultsViewMode {
  return value === 'map' ? 'map' : 'list';
}
