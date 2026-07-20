use axum::{
    extract::{rejection::PathRejection, Path, State},
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    http::{
        error::ApiError,
        validation::{ValidatedJson, ValidateRequest, ValidationError},
        AppState,
    },
    services::status_change::{
        self as status_change_service, StatusChangeResult, StatusChangeSubmission,
    },
};

const MAX_NOTE_CHARS: usize = 1000;

#[derive(Debug, Deserialize)]
pub struct StatusChangePath {
    doctor_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct StatusChangeRequest {
    note: Option<String>,
}

impl ValidateRequest for StatusChangeRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(note) = &self.note {
            let trimmed = note.trim();
            if trimmed.is_empty() {
                return Err(ValidationError::field(
                    "note",
                    "note must not be blank when provided",
                ));
            }

            if trimmed.chars().count() > MAX_NOTE_CHARS {
                return Err(ValidationError::field(
                    "note",
                    format!("note must be {MAX_NOTE_CHARS} characters or fewer"),
                ));
            }
        }

        Ok(())
    }
}

pub async fn report_status_change(
    State(state): State<AppState>,
    path: Result<Path<StatusChangePath>, PathRejection>,
    ValidatedJson(request): ValidatedJson<StatusChangeRequest>,
) -> Result<Json<StatusChangeResult>, ApiError> {
    let Path(path) = path?;
    let doctor_id = validate_doctor_id(path.doctor_id)?;
    let submission = StatusChangeSubmission {
        note: request.note.map(|note| note.trim().to_string()),
    };

    let result = status_change_service::report_status_change(
        &state.pool,
        doctor_id,
        submission,
        Utc::now(),
    )
    .await
    .map_err(|source| {
        ApiError::service_unavailable("status change submission unavailable", source)
    })?;

    let Some(result) = result else {
        return Err(ApiError::resource_not_found(
            "doctor_not_found",
            format!("No doctor listing found for id {doctor_id}"),
        ));
    };

    Ok(Json(result))
}

fn validate_doctor_id(doctor_id: i64) -> Result<i64, ApiError> {
    if doctor_id <= 0 {
        return Err(ApiError::bad_request(
            "invalid_doctor_id",
            "doctor_id must be greater than zero",
        ));
    }

    Ok(doctor_id)
}
