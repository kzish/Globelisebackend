//! Error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Errors that can occurr during registration.
#[derive(Debug, Serialize)]
pub struct RegistrationError {
    pub is_valid_email: bool,
    pub is_email_available: bool,
    pub is_password_at_least_8_chars: bool,
    pub passwords_match: bool,
}

/// Error responses.
#[derive(Debug)]
pub enum Error {
    Registration(RegistrationError),
    Dapr(String),
    GooglePublicKeys,
    Conversion(String),
    Unauthorized,
    BadRequest,
    Internal,
    InternalVerbose(String),
}

#[cfg(debug_assertions)]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Registration(error) => {
                return (StatusCode::BAD_REQUEST, Json(error)).into_response();
            }
            Error::Dapr(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::GooglePublicKeys => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get Google's public keys".into(),
            ),
            Error::Conversion(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Error::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".into()),
            Error::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
            Error::InternalVerbose(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        (status, message).into_response()
    }
}

#[cfg(not(debug_assertions))]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Registration(error) => {
                return (StatusCode::BAD_REQUEST, Json(error)).into_response();
            }
            Error::Dapr(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::GooglePublicKeys => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Conversion(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            Error::Internal | Error::InternalVerbose(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        (status, message).into_response()
    }
}
