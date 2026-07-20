#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "availability_report_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityReportStatus {
    Accepting,
    NotAccepting,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "availability_report_kind", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityReportKind {
    ConfirmAccepting,
    StatusChange,
}

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct AvailabilityReport {
    pub id: i64,
    pub family_doctor_id: i64,
    pub report_kind: AvailabilityReportKind,
    pub reported_status: AvailabilityReportStatus,
    pub note: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewAvailabilityReport {
    pub family_doctor_id: i64,
    pub report_kind: AvailabilityReportKind,
    pub reported_status: AvailabilityReportStatus,
    pub note: Option<String>,
}
