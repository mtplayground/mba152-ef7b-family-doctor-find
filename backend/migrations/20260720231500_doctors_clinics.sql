CREATE TABLE clinics (
    id BIGSERIAL PRIMARY KEY,
    city_area_id BIGINT NOT NULL REFERENCES city_areas (id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    address_line1 TEXT NOT NULL,
    address_line2 TEXT,
    municipality TEXT NOT NULL,
    province_code CHAR(2) NOT NULL,
    postal_code TEXT,
    phone TEXT,
    fax TEXT,
    email TEXT,
    website_url TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT clinics_name_not_blank CHECK (BTRIM(name) <> ''),
    CONSTRAINT clinics_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT clinics_address_line1_not_blank CHECK (BTRIM(address_line1) <> ''),
    CONSTRAINT clinics_address_line2_not_blank CHECK (
        address_line2 IS NULL OR BTRIM(address_line2) <> ''
    ),
    CONSTRAINT clinics_municipality_not_blank CHECK (BTRIM(municipality) <> ''),
    CONSTRAINT clinics_province_code_format CHECK (province_code ~ '^[A-Z]{2}$'),
    CONSTRAINT clinics_postal_code_format CHECK (
        postal_code IS NULL OR postal_code ~* '^[ABCEGHJ-NPRSTVXY][0-9][ABCEGHJ-NPRSTV-Z][ -]?[0-9][ABCEGHJ-NPRSTV-Z][0-9]$'
    ),
    CONSTRAINT clinics_phone_not_blank CHECK (phone IS NULL OR BTRIM(phone) <> ''),
    CONSTRAINT clinics_fax_not_blank CHECK (fax IS NULL OR BTRIM(fax) <> ''),
    CONSTRAINT clinics_email_format CHECK (
        email IS NULL OR email ~* '^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$'
    ),
    CONSTRAINT clinics_website_url_format CHECK (
        website_url IS NULL OR website_url ~* '^https?://'
    ),
    CONSTRAINT clinics_latitude_range CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CONSTRAINT clinics_longitude_range CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180)
);

CREATE UNIQUE INDEX clinics_city_area_slug_unique ON clinics (city_area_id, slug);
CREATE INDEX clinics_city_area_id_idx ON clinics (city_area_id);
CREATE INDEX clinics_name_search_idx ON clinics (LOWER(name));
CREATE INDEX clinics_municipality_idx ON clinics (municipality, province_code);

CREATE TABLE family_doctors (
    id BIGSERIAL PRIMARY KEY,
    clinic_id BIGINT NOT NULL REFERENCES clinics (id) ON DELETE RESTRICT,
    full_name TEXT NOT NULL,
    slug TEXT NOT NULL,
    credentials TEXT,
    phone TEXT,
    email TEXT,
    profile_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT family_doctors_full_name_not_blank CHECK (BTRIM(full_name) <> ''),
    CONSTRAINT family_doctors_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT family_doctors_credentials_not_blank CHECK (
        credentials IS NULL OR BTRIM(credentials) <> ''
    ),
    CONSTRAINT family_doctors_phone_not_blank CHECK (phone IS NULL OR BTRIM(phone) <> ''),
    CONSTRAINT family_doctors_email_format CHECK (
        email IS NULL OR email ~* '^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$'
    ),
    CONSTRAINT family_doctors_profile_url_format CHECK (
        profile_url IS NULL OR profile_url ~* '^https?://'
    )
);

CREATE UNIQUE INDEX family_doctors_clinic_slug_unique ON family_doctors (clinic_id, slug);
CREATE INDEX family_doctors_clinic_id_idx ON family_doctors (clinic_id);
CREATE INDEX family_doctors_name_search_idx ON family_doctors (LOWER(full_name));

CREATE TRIGGER clinics_set_updated_at
BEFORE UPDATE ON clinics
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER family_doctors_set_updated_at
BEFORE UPDATE ON family_doctors
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
