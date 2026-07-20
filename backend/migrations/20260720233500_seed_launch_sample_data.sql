WITH seed_cities (name, province_code, slug, latitude, longitude) AS (
    VALUES
        ('Toronto', 'ON', 'toronto', 43.6532, -79.3832),
        ('Vancouver', 'BC', 'vancouver', 49.2827, -123.1207),
        ('Calgary', 'AB', 'calgary', 51.0447, -114.0719),
        ('Montreal', 'QC', 'montreal', 45.5017, -73.5673),
        ('Ottawa', 'ON', 'ottawa', 45.4215, -75.6972),
        ('Halifax', 'NS', 'halifax', 44.6488, -63.5752)
)
INSERT INTO cities (name, province_code, slug, latitude, longitude)
SELECT name, province_code, slug, latitude, longitude
FROM seed_cities
ON CONFLICT (slug) DO UPDATE
SET
    name = EXCLUDED.name,
    province_code = EXCLUDED.province_code,
    latitude = EXCLUDED.latitude,
    longitude = EXCLUDED.longitude;

WITH seed_areas (city_slug, name, slug, latitude, longitude) AS (
    VALUES
        ('toronto', 'Downtown Toronto', 'downtown-toronto', 43.6548, -79.3883),
        ('toronto', 'North York', 'north-york', 43.7615, -79.4111),
        ('toronto', 'Scarborough', 'scarborough', 43.7731, -79.2578),
        ('vancouver', 'Downtown Vancouver', 'downtown-vancouver', 49.2819, -123.1199),
        ('vancouver', 'Kitsilano', 'kitsilano', 49.2684, -123.1683),
        ('vancouver', 'East Vancouver', 'east-vancouver', 49.2550, -123.0689),
        ('calgary', 'Beltline', 'beltline', 51.0415, -114.0754),
        ('calgary', 'Northwest Calgary', 'northwest-calgary', 51.0802, -114.1301),
        ('montreal', 'Plateau Mont-Royal', 'plateau-mont-royal', 45.5216, -73.5800),
        ('montreal', 'Ville-Marie', 'ville-marie', 45.5088, -73.5540),
        ('ottawa', 'Centretown', 'centretown', 45.4153, -75.6950),
        ('ottawa', 'Kanata', 'kanata', 45.3088, -75.8987),
        ('halifax', 'Downtown Halifax', 'downtown-halifax', 44.6460, -63.5730),
        ('halifax', 'Clayton Park', 'clayton-park', 44.6654, -63.6506)
)
INSERT INTO city_areas (city_id, name, slug, latitude, longitude)
SELECT cities.id, seed_areas.name, seed_areas.slug, seed_areas.latitude, seed_areas.longitude
FROM seed_areas
JOIN cities ON cities.slug = seed_areas.city_slug
ON CONFLICT (city_id, slug) DO UPDATE
SET
    name = EXCLUDED.name,
    latitude = EXCLUDED.latitude,
    longitude = EXCLUDED.longitude;

WITH seed_clinics (
    city_slug,
    area_slug,
    name,
    slug,
    address_line1,
    municipality,
    province_code,
    postal_code,
    phone,
    website_url,
    latitude,
    longitude
) AS (
    VALUES
        (
            'toronto',
            'downtown-toronto',
            'Sample Toronto Family Clinic',
            'sample-toronto-family-clinic',
            '100 Sample Street',
            'Toronto',
            'ON',
            'M5H 2N2',
            '+1-416-555-0100',
            'https://example.com/sample-toronto-family-clinic',
            43.6548,
            -79.3883
        ),
        (
            'toronto',
            'north-york',
            'Sample North York Primary Care',
            'sample-north-york-primary-care',
            '200 Sample Avenue',
            'Toronto',
            'ON',
            'M2N 5V7',
            '+1-416-555-0110',
            'https://example.com/sample-north-york-primary-care',
            43.7615,
            -79.4111
        ),
        (
            'vancouver',
            'downtown-vancouver',
            'Sample Vancouver Family Practice',
            'sample-vancouver-family-practice',
            '300 Sample Road',
            'Vancouver',
            'BC',
            'V6B 2W9',
            '+1-604-555-0100',
            'https://example.com/sample-vancouver-family-practice',
            49.2819,
            -123.1199
        ),
        (
            'calgary',
            'beltline',
            'Sample Calgary Family Health',
            'sample-calgary-family-health',
            '400 Sample Trail',
            'Calgary',
            'AB',
            'T2P 1J9',
            '+1-403-555-0100',
            'https://example.com/sample-calgary-family-health',
            51.0415,
            -114.0754
        ),
        (
            'montreal',
            'ville-marie',
            'Sample Montreal Family Clinic',
            'sample-montreal-family-clinic',
            '500 Rue Sample',
            'Montreal',
            'QC',
            'H2X 1Y4',
            '+1-514-555-0100',
            'https://example.com/sample-montreal-family-clinic',
            45.5088,
            -73.5540
        ),
        (
            'ottawa',
            'centretown',
            'Sample Ottawa Primary Care',
            'sample-ottawa-primary-care',
            '600 Sample Drive',
            'Ottawa',
            'ON',
            'K1P 1J1',
            '+1-613-555-0100',
            'https://example.com/sample-ottawa-primary-care',
            45.4153,
            -75.6950
        ),
        (
            'halifax',
            'downtown-halifax',
            'Sample Halifax Family Practice',
            'sample-halifax-family-practice',
            '700 Sample Lane',
            'Halifax',
            'NS',
            'B3J 1S9',
            '+1-902-555-0100',
            'https://example.com/sample-halifax-family-practice',
            44.6460,
            -63.5730
        )
)
INSERT INTO clinics (
    city_area_id,
    name,
    slug,
    address_line1,
    municipality,
    province_code,
    postal_code,
    phone,
    website_url,
    latitude,
    longitude
)
SELECT
    city_areas.id,
    seed_clinics.name,
    seed_clinics.slug,
    seed_clinics.address_line1,
    seed_clinics.municipality,
    seed_clinics.province_code,
    seed_clinics.postal_code,
    seed_clinics.phone,
    seed_clinics.website_url,
    seed_clinics.latitude,
    seed_clinics.longitude
FROM seed_clinics
JOIN cities ON cities.slug = seed_clinics.city_slug
JOIN city_areas
    ON city_areas.city_id = cities.id
    AND city_areas.slug = seed_clinics.area_slug
ON CONFLICT (city_area_id, slug) DO UPDATE
SET
    name = EXCLUDED.name,
    address_line1 = EXCLUDED.address_line1,
    municipality = EXCLUDED.municipality,
    province_code = EXCLUDED.province_code,
    postal_code = EXCLUDED.postal_code,
    phone = EXCLUDED.phone,
    website_url = EXCLUDED.website_url,
    latitude = EXCLUDED.latitude,
    longitude = EXCLUDED.longitude;

WITH seed_doctors (clinic_slug, full_name, slug, credentials, phone, profile_url) AS (
    VALUES
        (
            'sample-toronto-family-clinic',
            'Dr. Avery Morgan (Sample Listing)',
            'avery-morgan-sample',
            'MD, CCFP',
            '+1-416-555-0101',
            'https://example.com/sample-toronto-family-clinic/avery-morgan'
        ),
        (
            'sample-north-york-primary-care',
            'Dr. Jordan Patel (Sample Listing)',
            'jordan-patel-sample',
            'MD, CCFP',
            '+1-416-555-0111',
            'https://example.com/sample-north-york-primary-care/jordan-patel'
        ),
        (
            'sample-vancouver-family-practice',
            'Dr. Riley Chen (Sample Listing)',
            'riley-chen-sample',
            'MD, CCFP',
            '+1-604-555-0101',
            'https://example.com/sample-vancouver-family-practice/riley-chen'
        ),
        (
            'sample-calgary-family-health',
            'Dr. Taylor Singh (Sample Listing)',
            'taylor-singh-sample',
            'MD, CCFP',
            '+1-403-555-0101',
            'https://example.com/sample-calgary-family-health/taylor-singh'
        ),
        (
            'sample-montreal-family-clinic',
            'Dr. Casey Nguyen (Sample Listing)',
            'casey-nguyen-sample',
            'MD, CCFP',
            '+1-514-555-0101',
            'https://example.com/sample-montreal-family-clinic/casey-nguyen'
        ),
        (
            'sample-ottawa-primary-care',
            'Dr. Rowan Brooks (Sample Listing)',
            'rowan-brooks-sample',
            'MD, CCFP',
            '+1-613-555-0101',
            'https://example.com/sample-ottawa-primary-care/rowan-brooks'
        ),
        (
            'sample-halifax-family-practice',
            'Dr. Morgan Lee (Sample Listing)',
            'morgan-lee-sample',
            'MD, CCFP',
            '+1-902-555-0101',
            'https://example.com/sample-halifax-family-practice/morgan-lee'
        )
)
INSERT INTO family_doctors (
    clinic_id,
    full_name,
    slug,
    credentials,
    phone,
    profile_url
)
SELECT
    clinics.id,
    seed_doctors.full_name,
    seed_doctors.slug,
    seed_doctors.credentials,
    seed_doctors.phone,
    seed_doctors.profile_url
FROM seed_doctors
JOIN clinics ON clinics.slug = seed_doctors.clinic_slug
ON CONFLICT (clinic_id, slug) DO UPDATE
SET
    full_name = EXCLUDED.full_name,
    credentials = EXCLUDED.credentials,
    phone = EXCLUDED.phone,
    profile_url = EXCLUDED.profile_url;

WITH seed_reports (doctor_slug, reported_status, submitted_at) AS (
    VALUES
        ('avery-morgan-sample', 'accepting'::availability_report_status, NOW() - INTERVAL '2 days'),
        ('jordan-patel-sample', 'unknown'::availability_report_status, NOW() - INTERVAL '5 days'),
        ('riley-chen-sample', 'not_accepting'::availability_report_status, NOW() - INTERVAL '3 days'),
        ('taylor-singh-sample', 'accepting'::availability_report_status, NOW() - INTERVAL '1 day'),
        ('casey-nguyen-sample', 'accepting'::availability_report_status, NOW() - INTERVAL '4 days'),
        ('rowan-brooks-sample', 'unknown'::availability_report_status, NOW() - INTERVAL '6 days'),
        ('morgan-lee-sample', 'accepting'::availability_report_status, NOW() - INTERVAL '2 days')
)
INSERT INTO availability_reports (
    family_doctor_id,
    report_kind,
    reported_status,
    note,
    submitted_at
)
SELECT
    family_doctors.id,
    CASE
        WHEN seed_reports.reported_status = 'accepting' THEN 'confirm_accepting'::availability_report_kind
        ELSE 'status_change'::availability_report_kind
    END,
    seed_reports.reported_status,
    'Seed sample report for launch data',
    seed_reports.submitted_at
FROM seed_reports
JOIN family_doctors ON family_doctors.slug = seed_reports.doctor_slug
WHERE NOT EXISTS (
    SELECT 1
    FROM availability_reports existing_reports
    WHERE existing_reports.family_doctor_id = family_doctors.id
);
