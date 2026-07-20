use axum::{
    extract::{rejection::PathRejection, Path, State},
    http::HeaderMap,
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    http::{
        error::ApiError,
        rate_limit,
        validation::{ValidatedJson, ValidateRequest, ValidationError},
        AppState,
    },
    services::confirm_accepting::{
        self as confirm_accepting_service, ConfirmAcceptingResult, ConfirmAcceptingSubmission,
    },
};

const MAX_NOTE_CHARS: usize = 1000;

#[derive(Debug, Deserialize)]
pub struct ConfirmAcceptingPath {
    doctor_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmAcceptingRequest {
    note: Option<String>,
}

impl ValidateRequest for ConfirmAcceptingRequest {
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

pub async fn confirm_accepting(
    State(state): State<AppState>,
    path: Result<Path<ConfirmAcceptingPath>, PathRejection>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<ConfirmAcceptingRequest>,
) -> Result<Json<ConfirmAcceptingResult>, ApiError> {
    let Path(path) = path?;
    let doctor_id = validate_doctor_id(path.doctor_id)?;
    let client_id = rate_limit::client_identifier(&headers);
    state
        .rate_limiter
        .check_listing_report(&client_id, doctor_id)
        .await?;
    state.rate_limiter.check_submission(&client_id).await?;

    let submission = ConfirmAcceptingSubmission {
        note: request.note.map(|note| note.trim().to_string()),
    };

    let result = confirm_accepting_service::confirm_accepting(
        &state.pool,
        doctor_id,
        submission,
        Utc::now(),
    )
    .await
    .map_err(|source| {
        ApiError::service_unavailable("confirm accepting submission unavailable", source)
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
