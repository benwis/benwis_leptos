use cfg_if::cfg_if;
use http::status::StatusCode;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize)]
pub enum BenwisAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("SqlxError: {0}")]
    SqlxError(String),
    #[error("Argon2Error: {0}")]
    Argon2Error(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("Authentication required")]
    AuthError,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Json Error: {0}")]
    JsonError(String),
    #[error("Server Error: {0}")]
    ServerError(String),
}

impl BenwisAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            BenwisAppError::NotFound => StatusCode::NOT_FOUND,
            BenwisAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::Argon2Error(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            BenwisAppError::AuthError => StatusCode::UNAUTHORIZED,
            BenwisAppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::JsonError(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl From<serde_wasm_bindgen::Error> for BenwisAppError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        BenwisAppError::JsonError(e.to_string())
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
    }
}
