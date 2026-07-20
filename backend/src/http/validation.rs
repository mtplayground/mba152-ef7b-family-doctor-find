#![allow(dead_code)]

use axum::{
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use std::{error::Error, fmt};

use crate::http::error::ApiError;

pub trait ValidateRequest {
    fn validate(&self) -> Result<(), ValidationError>;
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + ValidateRequest,
{
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await?;
        value.validate()?;

        Ok(Self(value))
    }
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    field: Option<&'static str>,
    message: String,
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            field: None,
            message: message.into(),
        }
    }

    pub fn field(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            field: Some(field),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.field {
            Some(field) => write!(formatter, "{field}: {}", self.message),
            None => write!(formatter, "{}", self.message),
        }
    }
}

impl Error for ValidationError {}
