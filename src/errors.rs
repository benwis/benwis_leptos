use http::status::StatusCode;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum BenwisAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl BenwisAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            BenwisAppError::NotFound => StatusCode::NOT_FOUND,
            BenwisAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
