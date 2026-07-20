CREATE TABLE cities (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    province_code CHAR(2) NOT NULL,
    country_code CHAR(2) NOT NULL DEFAULT 'CA',
    slug TEXT NOT NULL,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT cities_name_not_blank CHECK (BTRIM(name) <> ''),
    CONSTRAINT cities_province_code_format CHECK (province_code ~ '^[A-Z]{2}$'),
    CONSTRAINT cities_country_code_ca CHECK (country_code = 'CA'),
    CONSTRAINT cities_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT cities_latitude_range CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CONSTRAINT cities_longitude_range CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180)
);

CREATE UNIQUE INDEX cities_slug_unique ON cities (slug);
CREATE UNIQUE INDEX cities_name_province_unique ON cities (LOWER(name), province_code);
CREATE INDEX cities_province_code_idx ON cities (province_code);

CREATE TABLE city_areas (
    id BIGSERIAL PRIMARY KEY,
    city_id BIGINT NOT NULL REFERENCES cities (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT city_areas_name_not_blank CHECK (BTRIM(name) <> ''),
    CONSTRAINT city_areas_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT city_areas_latitude_range CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CONSTRAINT city_areas_longitude_range CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180)
);

CREATE UNIQUE INDEX city_areas_city_slug_unique ON city_areas (city_id, slug);
CREATE UNIQUE INDEX city_areas_city_name_unique ON city_areas (city_id, LOWER(name));
CREATE INDEX city_areas_city_id_idx ON city_areas (city_id);

CREATE FUNCTION set_updated_at()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;

CREATE TRIGGER cities_set_updated_at
BEFORE UPDATE ON cities
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER city_areas_set_updated_at
BEFORE UPDATE ON city_areas
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
