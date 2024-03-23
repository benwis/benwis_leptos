use cfg_if::cfg_if;
use http::status::StatusCode;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize)]
pub enum BenwisAppError {
    #[error("Not Found")]
    NotFound,
    #[error("AuthError")]
    AuthError,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("SqlxError: {0}")]
    SqlxError(String),
    #[error("Argon2Error: {0}")]
    Argon2Error(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("SessionError: {0}")]
    SessionError(String),
    #[error("JsonError: {0}")]
    JsonError(String),
}

impl BenwisAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            BenwisAppError::NotFound => StatusCode::NOT_FOUND,
            BenwisAppError::AuthError => StatusCode::NOT_FOUND,
            BenwisAppError::InternalServerError => StatusCode::UNAUTHORIZED,
            BenwisAppError::SessionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::JsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::Argon2Error(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl From<sqlx::Error> for BenwisAppError {
            fn from(value: sqlx::Error) -> Self {
                Self::SqlxError(value.to_string())
            }
        }
        impl From<argon2::password_hash::Error> for BenwisAppError {
            fn from(error: argon2::password_hash::Error) -> Self {
                Self::Argon2Error(error.to_string())
            }
        }
        impl From<async_session::Error> for BenwisAppError{
            fn from(error: async_session::Error) -> Self {
                Self::SessionError(error.to_string())
            }
        }
        impl From<serde_json::Error> for BenwisAppError{
            fn from(error: serde_json::Error) -> Self {
                Self::JsonError(error.to_string())
            }
        }
    }
}
