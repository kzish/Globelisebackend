//! Error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Error responses.
#[derive(Debug)]
pub enum Error {
    Unauthorized(&'static str),
    #[allow(dead_code)]
    NotFound,
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized(message) => {
                println!("{message}");
                StatusCode::UNAUTHORIZED.into_response()
            }
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::Internal(message) => {
                eprintln!("{message}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error,
{
    fn from(e: T) -> Self {
        Error::Internal(e.to_string())
    }
}
