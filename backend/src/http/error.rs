use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::{error::Error, fmt};

use crate::http::{rate_limit::RateLimitError, validation::ValidationError};

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
    ResourceNotFound {
        code: &'static str,
        message: String,
    },
    RateLimited {
        retry_after_secs: u64,
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

    pub fn resource_not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::ResourceNotFound {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest { code, message } => write!(formatter, "{code}: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
            Self::NotFound { path } => write!(formatter, "not found: {path}"),
            Self::ResourceNotFound { code, message } => write!(formatter, "{code}: {message}"),
            Self::RateLimited { retry_after_secs } => {
                write!(formatter, "rate_limited: retry after {retry_after_secs} seconds")
            }
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
            Self::ResourceNotFound { code, message } => {
                (StatusCode::NOT_FOUND, *code, message.clone())
            }
            Self::RateLimited { retry_after_secs } => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited",
                format!("Too many submissions; try again in {retry_after_secs} seconds"),
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

impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        Self::bad_request("invalid_query", rejection.to_string())
    }
}

impl From<PathRejection> for ApiError {
    fn from(rejection: PathRejection) -> Self {
        Self::bad_request("invalid_path", rejection.to_string())
    }
}

impl From<RateLimitError> for ApiError {
    fn from(error: RateLimitError) -> Self {
        Self::RateLimited {
            retry_after_secs: error.retry_after_secs,
        }
    }
}

pub async fn not_found(uri: Uri) -> ApiError {
    ApiError::NotFound {
        path: uri.path().to_string(),
    }
}
