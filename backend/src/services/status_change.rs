use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    db::{availability_report::AvailabilityReport, DbPool},
    services::{
        doctor_detail::ReportHistoryItem,
        listing_status::{self, ListingStatus},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusChangeSubmission {
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusChangeResult {
    pub doctor_id: i64,
    pub report: ReportHistoryItem,
    pub status: ListingStatus,
}

pub async fn report_status_change(
    pool: &DbPool,
    doctor_id: i64,
    submission: StatusChangeSubmission,
    now: DateTime<Utc>,
) -> Result<Option<StatusChangeResult>, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM family_doctors
            WHERE id = $1
        )
        "#,
    )
    .bind(doctor_id)
    .fetch_one(pool)
    .await?;

    if !exists {
        return Ok(None);
    }

    let report = sqlx::query_as::<_, AvailabilityReport>(
        r#"
        INSERT INTO availability_reports (
            family_doctor_id,
            report_kind,
            reported_status,
            note
        )
        VALUES ($1, 'status_change', 'not_accepting', $2)
        RETURNING id, family_doctor_id, report_kind, reported_status, note, submitted_at
        "#,
    )
    .bind(doctor_id)
    .bind(submission.note)
    .fetch_one(pool)
    .await?;

    let status = listing_status::load_listing_status(pool, doctor_id, now).await?;

    Ok(Some(StatusChangeResult {
        doctor_id,
        report: ReportHistoryItem::from(report),
        status,
    }))
}
