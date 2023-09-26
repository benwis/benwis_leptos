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
    #[error("TomlError: {0}")]
    TomlError(String),
    #[error("Argon2Error: {0}")]
    Argon2Error(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("Missing or Invalid Frontmatter")]
    MissingOrInvalidFrontmatter,
}

impl BenwisAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            BenwisAppError::NotFound => StatusCode::NOT_FOUND,
            BenwisAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::Argon2Error(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            BenwisAppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
         impl From<toml::de::Error> for BenwisAppError {
            fn from(value: toml::de::Error) -> Self {
                Self::TomlError(value.to_string())
            }
        }
        impl From<argon2::password_hash::Error> for BenwisAppError {
            fn from(error: argon2::password_hash::Error) -> Self {
                Self::Argon2Error(error.to_string())
            }
        }
    }
}
