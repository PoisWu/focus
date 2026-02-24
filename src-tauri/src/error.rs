use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Unsplash API error: {0}")]
    Unsplash(String),

    #[error("MPRIS error: {0}")]
    Mpris(String),

    #[error("No music player found")]
    NoPlayer,
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Unsplash(err.to_string())
    }
}

impl From<mpris::FindingError> for AppError {
    fn from(err: mpris::FindingError) -> Self {
        match err {
            mpris::FindingError::NoPlayerFound => AppError::NoPlayer,
            _ => AppError::Mpris(err.to_string()),
        }
    }
}

impl From<mpris::DBusError> for AppError {
    fn from(err: mpris::DBusError) -> Self {
        AppError::Mpris(err.to_string())
    }
}
