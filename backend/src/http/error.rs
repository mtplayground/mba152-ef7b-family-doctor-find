use axum::{
    extract::rejection::JsonRejection,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::{error::Error, fmt};

use crate::http::validation::ValidationError;

type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest {
        code: &'static str,
        message: String,
    },
    Validation(ValidationError),
    NotFound {
        path: String,
    },
    ServiceUnavailable {
        message: String,
        source: BoxError,
    },
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code,
            message: message.into(),
        }
    }

    pub fn service_unavailable(
        message: impl Into<String>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
            source: Box::new(source),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest { code, message } => write!(formatter, "{code}: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
            Self::NotFound { path } => write!(formatter, "not found: {path}"),
            Self::ServiceUnavailable { message, source } => {
                write!(formatter, "{message}: {source}")
            }
        }
    }
}

impl Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            Self::BadRequest { code, message } => {
                (StatusCode::BAD_REQUEST, *code, message.clone())
            }
            Self::Validation(error) => (
                StatusCode::BAD_REQUEST,
                "validation_failed",
                error.to_string(),
            ),
            Self::NotFound { path } => (
                StatusCode::NOT_FOUND,
                "not_found",
                format!("No route found for {path}"),
            ),
            Self::ServiceUnavailable { message, .. } => (
                StatusCode::SERVICE_UNAVAILABLE,
                "service_unavailable",
                message.clone(),
            ),
        };

        if status.is_server_error() {
            tracing::error!(error = ?self, "request failed");
        }

        (
            status,
            Json(ErrorResponse {
                error: ErrorBody { code, message },
            }),
        )
            .into_response()
    }
}

impl From<ValidationError> for ApiError {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::bad_request("invalid_json", rejection.to_string())
    }
}

pub async fn not_found(uri: Uri) -> ApiError {
    ApiError::NotFound {
        path: uri.path().to_string(),
    }
}
