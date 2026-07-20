import { useEffect, useMemo } from 'react';
import { Link } from 'react-router-dom';
import {
  MapContainer,
  Marker,
  Popup,
  TileLayer,
  useMap,
} from 'react-leaflet';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';
import { ApiError } from '../api/client';
import type { DoctorListing, ListingCity } from '../api/types';
import { ReportControls } from './ReportControls';

interface DoctorResultsMapProps {
  listings: DoctorListing[];
  isLoading: boolean;
  error: Error | null;
  selectedLabel: string;
  selectedCitySlug: string;
  city: ListingCity | undefined;
}

interface MappedListing {
  listing: DoctorListing;
  position: [number, number];
}

export function DoctorResultsMap({
  listings,
  isLoading,
  error,
  selectedLabel,
  selectedCitySlug,
  city,
}: DoctorResultsMapProps) {
  const mappedListings = useMemo(() => getMappedListings(listings), [listings]);
  const center = getMapCenter(mappedListings, city);

  if (!selectedCitySlug) {
    return (
      <section className="rounded-lg border border-dashed border-civic-100 bg-civic-50 p-6 sm:p-8">
        <h2 className="text-xl font-semibold text-civic-900">
          Start with a city
        </h2>
        <p className="mt-3 max-w-2xl text-sm leading-6 text-ink-700">
          Search suggestions include launch cities and neighbourhood areas. Pick
          one to map matching family doctor listings.
        </p>
      </section>
    );
  }

  if (isLoading) {
    return (
      <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
        <h2 className="text-xl font-semibold text-ink-900">
          Loading map for {selectedLabel || selectedCitySlug}
        </h2>
        <div className="mt-6 h-[520px] animate-pulse rounded-lg bg-surface-muted" />
      </section>
    );
  }

  if (error) {
    return (
      <section className="rounded-lg border border-service-red/20 bg-white p-6 shadow-sm sm:p-8">
        <h2 className="text-xl font-semibold text-service-red">
          Map unavailable
        </h2>
        <p className="mt-3 text-sm leading-6 text-ink-700">
          {error instanceof ApiError
            ? error.message
            : 'The directory could not load listings for this map.'}
        </p>
      </section>
    );
  }

  return (
    <section className="rounded-lg border border-ink-100 bg-surface-raised p-4 shadow-sm sm:p-6">
      <div className="flex flex-col gap-2 px-1 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-sm font-semibold uppercase text-civic-700">Map</p>
          <h2 className="mt-2 text-2xl font-semibold text-ink-900">
            {selectedLabel || selectedCitySlug}
          </h2>
        </div>
        <p className="text-sm text-ink-500">
          {mappedListings.length} mapped of {listings.length}{' '}
          {listings.length === 1 ? 'listing' : 'listings'}
        </p>
      </div>

      {mappedListings.length === 0 ? (
        <p className="mt-6 rounded-lg bg-surface-muted px-4 py-3 text-sm text-ink-700">
          No clinic coordinates are available for this search yet.
        </p>
      ) : (
        <div className="mt-5 overflow-hidden rounded-lg border border-ink-100 bg-white">
          <MapContainer
            center={center}
            zoom={mappedListings.length === 1 ? 14 : 11}
            scrollWheelZoom={false}
            className="doctor-results-map"
          >
            <TileLayer
              attribution="&copy; OpenStreetMap contributors"
              url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            <FitMapToResults listings={mappedListings} city={city} />
            {mappedListings.map((item, index) => (
              <Marker
                key={item.listing.id}
                position={item.position}
                icon={markerIcon(item.listing, index)}
              >
                <Popup>
                  <div className="grid gap-1 text-sm">
                    <strong>{item.listing.fullName}</strong>
                    <span>{item.listing.clinic.name}</span>
                    <span>{item.listing.area.name}</span>
                    <span>{recencyLabel(item.listing)}</span>
                    <Link
                      to={`/doctors/${item.listing.id}`}
                      className="font-semibold text-civic-700"
                    >
                      Details
                    </Link>
                    <div className="mt-2">
                      <ReportControls doctorId={item.listing.id} compact />
                    </div>
                  </div>
                </Popup>
              </Marker>
            ))}
          </MapContainer>
        </div>
      )}
    </section>
  );
}

function FitMapToResults({
  listings,
  city,
}: {
  listings: MappedListing[];
  city: ListingCity | undefined;
}) {
  const map = useMap();

  useEffect(() => {
    if (listings.length > 1) {
      map.fitBounds(
        L.latLngBounds(listings.map((item) => item.position)),
        { padding: [36, 36] },
      );
      return;
    }

    if (listings.length === 1) {
      map.setView(listings[0].position, 14);
      return;
    }

    if (isCoordinate(city?.latitude) && isCoordinate(city?.longitude)) {
      map.setView([city.latitude, city.longitude], 11);
    }
  }, [city?.latitude, city?.longitude, listings, map]);

  return null;
}

function getMappedListings(listings: DoctorListing[]) {
  return listings.flatMap((listing) => {
    const latitude = listing.clinic.latitude;
    const longitude = listing.clinic.longitude;

    if (!isCoordinate(latitude) || !isCoordinate(longitude)) {
      return [];
    }

    return [{ listing, position: [latitude, longitude] as [number, number] }];
  });
}

function getMapCenter(
  listings: MappedListing[],
  city: ListingCity | undefined,
): [number, number] {
  if (listings.length > 0) {
    return listings[0].position;
  }

  if (isCoordinate(city?.latitude) && isCoordinate(city?.longitude)) {
    return [city.latitude, city.longitude];
  }

  return [56.1304, -106.3468];
}

function markerIcon(item: DoctorListing, index: number) {
  return L.divIcon({
    className: [
      'doctor-map-marker',
      item.status.current_status === 'accepting'
        ? 'doctor-map-marker-accepting'
        : 'doctor-map-marker-muted',
    ].join(' '),
    html: `<span>${index + 1}</span>`,
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
  });
}

function isCoordinate(value: number | null | undefined): value is number {
  return typeof value === 'number' && Number.isFinite(value);
}

function recencyLabel(listing: DoctorListing) {
  const daysAgo = listing.status.last_confirmed_accepting_days_ago;

  if (daysAgo === null) {
    return 'No accepting confirmation';
  }

  if (daysAgo === 0) {
    return 'Confirmed today';
  }

  if (daysAgo === 1) {
    return 'Confirmed 1 day ago';
  }

  return `Confirmed ${daysAgo} days ago`;
}
