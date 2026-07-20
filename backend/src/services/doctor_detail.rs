use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::{
    db::{
        availability_report::{
            AvailabilityReport, AvailabilityReportKind, AvailabilityReportStatus,
        },
        DbPool,
    },
    services::{
        doctor_listings::{ListingArea, ListingCity, ListingClinic},
        listing_status::{self, ListingStatus},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorDetail {
    pub id: i64,
    pub slug: String,
    pub full_name: String,
    pub credentials: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub profile_url: Option<String>,
    pub clinic: ListingClinic,
    pub area: ListingArea,
    pub city: ListingCity,
    pub status: ListingStatus,
    pub report_history: Vec<ReportHistoryItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportHistoryItem {
    pub id: i64,
    pub report_kind: AvailabilityReportKind,
    pub reported_status: AvailabilityReportStatus,
    pub note: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

impl From<AvailabilityReport> for ReportHistoryItem {
    fn from(report: AvailabilityReport) -> Self {
        Self {
            id: report.id,
            report_kind: report.report_kind,
            reported_status: report.reported_status,
            note: report.note,
            submitted_at: report.submitted_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromRow)]
struct DoctorDetailRow {
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
    city_id: i64,
    city_name: String,
    city_slug: String,
    city_province_code: String,
    city_latitude: Option<f64>,
    city_longitude: Option<f64>,
}

pub async fn get_doctor_detail(
    pool: &DbPool,
    doctor_id: i64,
    now: DateTime<Utc>,
) -> Result<Option<DoctorDetail>, sqlx::Error> {
    let row = sqlx::query_as::<_, DoctorDetailRow>(
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
            city_areas.slug AS area_slug,
            cities.id AS city_id,
            cities.name AS city_name,
            cities.slug AS city_slug,
            cities.province_code AS city_province_code,
            cities.latitude AS city_latitude,
            cities.longitude AS city_longitude
        FROM family_doctors
        JOIN clinics ON clinics.id = family_doctors.clinic_id
        JOIN city_areas ON city_areas.id = clinics.city_area_id
        JOIN cities ON cities.id = city_areas.city_id
        WHERE family_doctors.id = $1
        "#,
    )
    .bind(doctor_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let reports = load_report_history(pool, doctor_id).await?;
    let status = listing_status::derive_listing_status(doctor_id, &reports, now);

    Ok(Some(row.into_detail(status, reports)))
}

async fn load_report_history(
    pool: &DbPool,
    doctor_id: i64,
) -> Result<Vec<AvailabilityReport>, sqlx::Error> {
    sqlx::query_as::<_, AvailabilityReport>(
        r#"
        SELECT id, family_doctor_id, report_kind, reported_status, note, submitted_at
        FROM availability_reports
        WHERE family_doctor_id = $1
        ORDER BY submitted_at DESC, id DESC
        "#,
    )
    .bind(doctor_id)
    .fetch_all(pool)
    .await
}

impl DoctorDetailRow {
    fn into_detail(
        self,
        status: ListingStatus,
        reports: Vec<AvailabilityReport>,
    ) -> DoctorDetail {
        DoctorDetail {
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
            city: ListingCity {
                id: self.city_id,
                name: self.city_name,
                slug: self.city_slug,
                province_code: self.city_province_code,
                latitude: self.city_latitude,
                longitude: self.city_longitude,
            },
            status,
            report_history: reports
                .into_iter()
                .map(ReportHistoryItem::from)
                .collect(),
        }
    }
}
