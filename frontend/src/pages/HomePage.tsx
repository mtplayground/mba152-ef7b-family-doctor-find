import { useNavigate } from 'react-router-dom';
import type { CitySearchResult } from '../api/types';
import { CityTypeahead } from '../components/CityTypeahead';

export function HomePage() {
  const navigate = useNavigate();

  function handleSelect(result: CitySearchResult) {
    const params = new URLSearchParams();
    params.set('city', result.citySlug);
    params.set('label', result.label);

    if (result.areaSlug) {
      params.set('area', result.areaSlug);
    }

    navigate(`/results?${params}`);
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

        <CityTypeahead onSelect={handleSelect} />
      </section>

      <section className="rounded-lg border border-dashed border-civic-100 bg-civic-50 p-6 sm:p-8">
        <h2 className="text-xl font-semibold text-civic-900">
          Start with a city
        </h2>
        <p className="mt-3 max-w-2xl text-sm leading-6 text-ink-700">
          Search suggestions include launch cities and neighbourhood areas. Pick
          one to open a scannable list of family doctor listings.
        </p>
      </section>
    </div>
  );
}
