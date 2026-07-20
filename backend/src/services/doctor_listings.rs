use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::{
    db::DbPool,
    services::listing_status::{self, DerivedAvailabilityStatus, ListingStatus},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorListingCriteria {
    pub city_slug: String,
    pub area_slug: Option<String>,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CityDoctorListings {
    pub city: ListingCity,
    pub listings: Vec<DoctorListing>,
}

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ListingCity {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub province_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorListing {
    pub id: i64,
    pub slug: String,
    pub full_name: String,
    pub credentials: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub profile_url: Option<String>,
    pub clinic: ListingClinic,
    pub area: ListingArea,
    pub status: ListingStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingClinic {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub municipality: String,
    pub province_code: String,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingArea {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
struct DoctorListingRow {
    doctor_id: i64,
    doctor_slug: String,
    full_name: String,
    credentials: Option<String>,
    doctor_phone: Option<String>,
    doctor_email: Option<String>,
    profile_url: Option<String>,
    clinic_id: i64,
    clinic_name: String,
    clinic_slug: String,
    address_line1: String,
    address_line2: Option<String>,
    municipality: String,
    clinic_province_code: String,
    postal_code: Option<String>,
    clinic_phone: Option<String>,
    fax: Option<String>,
    clinic_email: Option<String>,
    website_url: Option<String>,
    clinic_latitude: Option<f64>,
    clinic_longitude: Option<f64>,
    area_id: i64,
    area_name: String,
    area_slug: String,
}

pub async fn list_doctors_by_city(
    pool: &DbPool,
    criteria: DoctorListingCriteria,
    now: DateTime<Utc>,
) -> Result<Option<CityDoctorListings>, sqlx::Error> {
    let city = sqlx::query_as::<_, ListingCity>(
        r#"
        SELECT id, name, slug, province_code, latitude, longitude
        FROM cities
        WHERE slug = $1
        "#,
    )
    .bind(&criteria.city_slug)
    .fetch_optional(pool)
    .await?;

    let Some(city) = city else {
        return Ok(None);
    };

    let rows = sqlx::query_as::<_, DoctorListingRow>(
        r#"
        SELECT
            family_doctors.id AS doctor_id,
            family_doctors.slug AS doctor_slug,
            family_doctors.full_name,
            family_doctors.credentials,
            family_doctors.phone AS doctor_phone,
            family_doctors.email AS doctor_email,
            family_doctors.profile_url,
            clinics.id AS clinic_id,
            clinics.name AS clinic_name,
            clinics.slug AS clinic_slug,
            clinics.address_line1,
            clinics.address_line2,
            clinics.municipality,
            clinics.province_code AS clinic_province_code,
            clinics.postal_code,
            clinics.phone AS clinic_phone,
            clinics.fax,
            clinics.email AS clinic_email,
            clinics.website_url,
            clinics.latitude AS clinic_latitude,
            clinics.longitude AS clinic_longitude,
            city_areas.id AS area_id,
            city_areas.name AS area_name,
            city_areas.slug AS area_slug
        FROM family_doctors
        JOIN clinics ON clinics.id = family_doctors.clinic_id
        JOIN city_areas ON city_areas.id = clinics.city_area_id
        WHERE city_areas.city_id = $1
            AND ($2::TEXT IS NULL OR city_areas.slug = $2)
        ORDER BY city_areas.name, clinics.name, family_doctors.full_name
        "#,
    )
    .bind(city.id)
    .bind(&criteria.area_slug)
    .fetch_all(pool)
    .await?;

    let doctor_ids = rows.iter().map(|row| row.doctor_id).collect::<Vec<_>>();
    let statuses = listing_status::load_listing_statuses(pool, &doctor_ids, now).await?;
    let statuses_by_doctor = statuses
        .into_iter()
        .map(|status| (status.family_doctor_id, status))
        .collect::<BTreeMap<_, _>>();

    let mut listings = rows
        .into_iter()
        .map(|row| {
            let status = statuses_by_doctor
                .get(&row.doctor_id)
                .cloned()
                .unwrap_or_else(|| listing_status::derive_listing_status(row.doctor_id, &[], now));
            row.into_listing(status)
        })
        .collect::<Vec<_>>();

    listings.sort_by(compare_listings);
    listings.truncate(criteria.limit);

    Ok(Some(CityDoctorListings { city, listings }))
}

impl DoctorListingRow {
    fn into_listing(self, status: ListingStatus) -> DoctorListing {
        DoctorListing {
            id: self.doctor_id,
            slug: self.doctor_slug,
            full_name: self.full_name,
            credentials: self.credentials,
            phone: self.doctor_phone,
            email: self.doctor_email,
            profile_url: self.profile_url,
            clinic: ListingClinic {
                id: self.clinic_id,
                name: self.clinic_name,
                slug: self.clinic_slug,
                address_line1: self.address_line1,
                address_line2: self.address_line2,
                municipality: self.municipality,
                province_code: self.clinic_province_code,
                postal_code: self.postal_code,
                phone: self.clinic_phone,
                fax: self.fax,
                email: self.clinic_email,
                website_url: self.website_url,
                latitude: self.clinic_latitude,
                longitude: self.clinic_longitude,
            },
            area: ListingArea {
                id: self.area_id,
                name: self.area_name,
                slug: self.area_slug,
            },
            status,
        }
    }
}

fn compare_listings(left: &DoctorListing, right: &DoctorListing) -> std::cmp::Ordering {
    listing_rank(left)
        .cmp(&listing_rank(right))
        .then_with(|| left.area.name.cmp(&right.area.name))
        .then_with(|| left.clinic.name.cmp(&right.clinic.name))
        .then_with(|| left.full_name.cmp(&right.full_name))
}

fn listing_rank(listing: &DoctorListing) -> (u8, i64) {
    let status_rank = match listing.status.current_status {
        DerivedAvailabilityStatus::Accepting => 0,
        DerivedAvailabilityStatus::Unknown => 1,
        DerivedAvailabilityStatus::NotAccepting => 2,
    };
    let recency_rank = listing
        .status
        .last_confirmed_accepting_days_ago
        .unwrap_or(i64::MAX);

    (status_rank, recency_rank)
}
