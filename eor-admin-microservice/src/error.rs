//! Error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Error responses.
#[derive(Debug)]
pub enum Error {
    Dapr(String),
    Database(String),
    UnavailableEmail,
    BadRequest(&'static str),
    Unauthorized(&'static str),
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Dapr(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Error::Database(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Error::UnavailableEmail => (StatusCode::CONFLICT, "Email is unavailable"),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::Unauthorized(message) => {
                eprintln!("{message}");
                return StatusCode::UNAUTHORIZED.into_response();
            }
            Error::Internal(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        (status, message).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e.to_string())
    }
}

impl From<email_address::Error> for Error {
    fn from(e: email_address::Error) -> Self {
        Error::Internal(e.to_string())
    }
}
