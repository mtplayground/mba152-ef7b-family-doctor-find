import {
  useEffect,
  useId,
  useState,
  type FormEvent,
  type KeyboardEvent,
} from 'react';
import { ApiError } from '../api/client';
import { useCitySearch } from '../api/hooks';
import type { CitySearchResult } from '../api/types';

interface CityTypeaheadProps {
  initialValue?: string;
  onSelect: (result: CitySearchResult) => void;
}

export function CityTypeahead({
  initialValue = '',
  onSelect,
}: CityTypeaheadProps) {
  const listboxId = useId();
  const [query, setQuery] = useState(initialValue);
  const [isOpen, setIsOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(0);
  const search = useCitySearch({ query, limit: 8 });
  const results = search.data?.results ?? [];
  const showResults = isOpen && query.trim().length > 0;

  useEffect(() => {
    setQuery(initialValue);
  }, [initialValue]);

  function selectResult(result: CitySearchResult) {
    setQuery(result.label);
    setIsOpen(false);
    setHighlightedIndex(0);
    onSelect(result);
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const result = results[highlightedIndex] ?? results[0];

    if (result) {
      selectResult(result);
    }
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === 'ArrowDown') {
      event.preventDefault();
      setIsOpen(true);
      setHighlightedIndex((current) =>
        results.length === 0 ? 0 : Math.min(current + 1, results.length - 1),
      );
    }

    if (event.key === 'ArrowUp') {
      event.preventDefault();
      setHighlightedIndex((current) => Math.max(current - 1, 0));
    }

    if (event.key === 'Escape') {
      setIsOpen(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} className="relative">
      <label
        htmlFor="city-search"
        className="text-sm font-semibold text-ink-800"
      >
        Search by city or area
      </label>
      <div className="mt-2 flex flex-col gap-3 sm:flex-row">
        <input
          id="city-search"
          value={query}
          type="search"
          autoComplete="off"
          role="combobox"
          aria-controls={listboxId}
          aria-expanded={showResults}
          aria-autocomplete="list"
          placeholder="Start with Toronto, Vancouver, Halifax..."
          className="min-h-12 flex-1 rounded-control border border-ink-100 bg-white px-4 text-base text-ink-900 shadow-sm outline-none transition placeholder:text-ink-500 focus:border-civic-500 focus:shadow-focus"
          onChange={(event) => {
            setQuery(event.target.value);
            setIsOpen(true);
            setHighlightedIndex(0);
          }}
          onFocus={() => setIsOpen(true)}
          onKeyDown={handleKeyDown}
        />
        <button
          type="submit"
          className="min-h-12 rounded-control bg-civic-700 px-5 text-sm font-semibold text-white shadow-sm transition hover:bg-civic-600 focus:outline-none focus-visible:shadow-focus"
        >
          Search
        </button>
      </div>

      {showResults ? (
        <div className="absolute left-0 right-0 z-20 mt-2 overflow-hidden rounded-lg border border-ink-100 bg-white shadow-lg">
          {search.isPending ? (
            <p className="px-4 py-3 text-sm text-ink-700">Searching...</p>
          ) : null}

          {search.isError ? (
            <div className="grid gap-3 px-4 py-3">
              <p className="text-sm font-semibold text-service-red">
                City search is unavailable.
              </p>
              <p className="text-sm leading-6 text-ink-700">
                {search.error instanceof ApiError
                  ? search.error.message
                  : 'Check your connection and try again.'}
              </p>
              <button
                type="button"
                className="w-fit rounded-control border border-service-red/30 bg-white px-3 py-2 text-xs font-semibold text-service-red transition hover:bg-service-red/10"
                onClick={() => void search.refetch()}
              >
                Try again
              </button>
            </div>
          ) : null}

          {!search.isPending && !search.isError && results.length === 0 ? (
            <div className="grid gap-1 px-4 py-3">
              <p className="text-sm font-semibold text-ink-800">
                No matching city or area found.
              </p>
              <p className="text-sm leading-6 text-ink-700">
                Try a larger nearby city, a different spelling, or the province
                abbreviation.
              </p>
            </div>
          ) : null}

          {results.length > 0 ? (
            <ul
              id={listboxId}
              role="listbox"
              className="max-h-80 overflow-y-auto py-1"
            >
              {results.map((result, index) => (
                <li
                  key={`${result.kind}-${result.citySlug}-${result.areaSlug ?? 'city'}`}
                >
                  <button
                    type="button"
                    role="option"
                    aria-selected={highlightedIndex === index}
                    className={[
                      'flex w-full items-start justify-between gap-4 px-4 py-3 text-left transition',
                      highlightedIndex === index
                        ? 'bg-civic-50 text-civic-900'
                        : 'text-ink-800 hover:bg-surface-muted',
                    ].join(' ')}
                    onMouseEnter={() => setHighlightedIndex(index)}
                    onClick={() => selectResult(result)}
                  >
                    <span>
                      <span className="block text-sm font-semibold">
                        {result.label}
                      </span>
                      <span className="mt-1 block text-xs uppercase text-ink-500">
                        {result.kind === 'area' ? 'Area' : 'City'}
                      </span>
                    </span>
                    <span className="text-xs font-semibold text-civic-700">
                      {result.provinceCode}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          ) : null}
        </div>
      ) : null}
    </form>
  );
}
