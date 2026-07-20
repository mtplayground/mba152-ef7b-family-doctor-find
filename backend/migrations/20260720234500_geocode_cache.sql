CREATE TABLE geocode_cache (
    query_key TEXT PRIMARY KEY,
    query_text TEXT NOT NULL,
    display_name TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    provider TEXT NOT NULL DEFAULT 'nominatim',
    provider_place_id BIGINT,
    provider_class TEXT,
    provider_type TEXT,
    importance DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT geocode_cache_query_key_not_blank CHECK (BTRIM(query_key) <> ''),
    CONSTRAINT geocode_cache_query_text_not_blank CHECK (BTRIM(query_text) <> ''),
    CONSTRAINT geocode_cache_display_name_not_blank CHECK (BTRIM(display_name) <> ''),
    CONSTRAINT geocode_cache_provider_not_blank CHECK (BTRIM(provider) <> ''),
    CONSTRAINT geocode_cache_latitude_range CHECK (latitude BETWEEN -90 AND 90),
    CONSTRAINT geocode_cache_longitude_range CHECK (longitude BETWEEN -180 AND 180)
);

CREATE INDEX geocode_cache_last_used_at_idx ON geocode_cache (last_used_at DESC);

CREATE TRIGGER geocode_cache_set_updated_at
BEFORE UPDATE ON geocode_cache
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
