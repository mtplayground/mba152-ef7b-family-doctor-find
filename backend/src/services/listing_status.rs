#![allow(dead_code)]

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::db::{
    availability_report::{AvailabilityReport, AvailabilityReportStatus},
    DbPool,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ListingStatus {
    pub family_doctor_id: i64,
    pub current_status: DerivedAvailabilityStatus,
    pub last_reported_at: Option<DateTime<Utc>>,
    pub last_confirmed_accepting_at: Option<DateTime<Utc>>,
    pub last_confirmed_accepting_days_ago: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedAvailabilityStatus {
    Accepting,
    NotAccepting,
    Unknown,
}

impl From<AvailabilityReportStatus> for DerivedAvailabilityStatus {
    fn from(status: AvailabilityReportStatus) -> Self {
        match status {
            AvailabilityReportStatus::Accepting => Self::Accepting,
            AvailabilityReportStatus::NotAccepting => Self::NotAccepting,
            AvailabilityReportStatus::Unknown => Self::Unknown,
        }
    }
}

pub fn derive_listing_status(
    family_doctor_id: i64,
    reports: &[AvailabilityReport],
    now: DateTime<Utc>,
) -> ListingStatus {
    let latest_report = reports
        .iter()
        .max_by_key(|report| (report.submitted_at, report.id));
    let latest_accepting_report = reports
        .iter()
        .filter(|report| report.reported_status == AvailabilityReportStatus::Accepting)
        .max_by_key(|report| (report.submitted_at, report.id));

    let last_confirmed_accepting_at =
        latest_accepting_report.map(|report| report.submitted_at);
    let last_confirmed_accepting_days_ago = last_confirmed_accepting_at
        .map(|submitted_at| now.signed_duration_since(submitted_at).num_days().max(0));

    ListingStatus {
        family_doctor_id,
        current_status: latest_report
            .map(|report| DerivedAvailabilityStatus::from(report.reported_status))
            .unwrap_or(DerivedAvailabilityStatus::Unknown),
        last_reported_at: latest_report.map(|report| report.submitted_at),
        last_confirmed_accepting_at,
        last_confirmed_accepting_days_ago,
    }
}

pub async fn load_listing_status(
    pool: &DbPool,
    family_doctor_id: i64,
    now: DateTime<Utc>,
) -> Result<ListingStatus, sqlx::Error> {
    let reports = sqlx::query_as::<_, AvailabilityReport>(
        r#"
        SELECT id, family_doctor_id, report_kind, reported_status, note, submitted_at
        FROM availability_reports
        WHERE family_doctor_id = $1
        ORDER BY submitted_at DESC, id DESC
        "#,
    )
    .bind(family_doctor_id)
    .fetch_all(pool)
    .await?;

    Ok(derive_listing_status(family_doctor_id, &reports, now))
}

pub async fn load_listing_statuses(
    pool: &DbPool,
    family_doctor_ids: &[i64],
    now: DateTime<Utc>,
) -> Result<Vec<ListingStatus>, sqlx::Error> {
    if family_doctor_ids.is_empty() {
        return Ok(Vec::new());
    }

    let reports = sqlx::query_as::<_, AvailabilityReport>(
        r#"
        SELECT id, family_doctor_id, report_kind, reported_status, note, submitted_at
        FROM availability_reports
        WHERE family_doctor_id = ANY($1)
        ORDER BY family_doctor_id ASC, submitted_at DESC, id DESC
        "#,
    )
    .bind(family_doctor_ids)
    .fetch_all(pool)
    .await?;

    let reports_by_doctor = reports.into_iter().fold(
        BTreeMap::<i64, Vec<AvailabilityReport>>::new(),
        |mut grouped, report| {
            grouped
                .entry(report.family_doctor_id)
                .or_default()
                .push(report);
            grouped
        },
    );

    Ok(family_doctor_ids
        .iter()
        .map(|family_doctor_id| {
            let reports = reports_by_doctor
                .get(family_doctor_id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            derive_listing_status(*family_doctor_id, reports, now)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use chrono::Duration;

    use super::*;
    use crate::db::availability_report::{AvailabilityReportKind, AvailabilityReportStatus};

    fn report(
        id: i64,
        family_doctor_id: i64,
        reported_status: AvailabilityReportStatus,
        submitted_at: DateTime<Utc>,
    ) -> AvailabilityReport {
        AvailabilityReport {
            id,
            family_doctor_id,
            report_kind: AvailabilityReportKind::StatusChange,
            reported_status,
            note: None,
            submitted_at,
        }
    }

    #[test]
    fn no_reports_returns_unknown_without_recency() {
        let now = fixed_now();

        let status = derive_listing_status(42, &[], now);

        assert_eq!(status.family_doctor_id, 42);
        assert_eq!(status.current_status, DerivedAvailabilityStatus::Unknown);
        assert_eq!(status.last_reported_at, None);
        assert_eq!(status.last_confirmed_accepting_days_ago, None);
    }

    #[test]
    fn latest_report_sets_current_status_and_accepting_recency() {
        let now = fixed_now();
        let accepting_at = now - Duration::days(3);
        let not_accepting_at = now - Duration::days(1);
        let reports = vec![
            report(1, 7, AvailabilityReportStatus::Accepting, accepting_at),
            report(2, 7, AvailabilityReportStatus::NotAccepting, not_accepting_at),
        ];

        let status = derive_listing_status(7, &reports, now);

        assert_eq!(
            status.current_status,
            DerivedAvailabilityStatus::NotAccepting
        );
        assert_eq!(status.last_reported_at, Some(not_accepting_at));
        assert_eq!(status.last_confirmed_accepting_at, Some(accepting_at));
        assert_eq!(status.last_confirmed_accepting_days_ago, Some(3));
    }

    #[test]
    fn future_confirmation_recency_is_clamped_to_zero() {
        let now = fixed_now();
        let reports = vec![report(
            1,
            7,
            AvailabilityReportStatus::Accepting,
            now + Duration::hours(2),
        )];

        let status = derive_listing_status(7, &reports, now);

        assert_eq!(status.last_confirmed_accepting_days_ago, Some(0));
    }

    fn fixed_now() -> DateTime<Utc> {
        DateTime::<Utc>::from(SystemTime::UNIX_EPOCH) + Duration::days(20_000)
    }
}
