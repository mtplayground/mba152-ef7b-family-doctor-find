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
pub struct ConfirmAcceptingSubmission {
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmAcceptingResult {
    pub doctor_id: i64,
    pub report: ReportHistoryItem,
    pub status: ListingStatus,
}

pub async fn confirm_accepting(
    pool: &DbPool,
    doctor_id: i64,
    submission: ConfirmAcceptingSubmission,
    now: DateTime<Utc>,
) -> Result<Option<ConfirmAcceptingResult>, sqlx::Error> {
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
        VALUES ($1, 'confirm_accepting', 'accepting', $2)
        RETURNING id, family_doctor_id, report_kind, reported_status, note, submitted_at
        "#,
    )
    .bind(doctor_id)
    .bind(submission.note)
    .fetch_one(pool)
    .await?;

    let status = listing_status::load_listing_status(pool, doctor_id, now).await?;

    Ok(Some(ConfirmAcceptingResult {
        doctor_id,
        report: ReportHistoryItem::from(report),
        status,
    }))
}
