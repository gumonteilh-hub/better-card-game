use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use std::fmt::{self};

/// The custom error type for this application.
///
/// It's designed to be serializable to be sent to the frontend.
#[derive(Debug, Serialize)]
pub enum Error {
    /// For errors coming from the game logic itself.
    Game(String),
    /// For when a Mutex is poisoned, indicating a panic elsewhere.
    MutexPoisoned,
    /// For errors related to JSON serialization/deserialization.
    Json(String),
    GameNotStarted,
}

// Implementation of the `Display` trait for human-readable error messages.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Game(msg) => write!(f, "Game Logic Error: {}", msg),
            Error::MutexPoisoned => write!(
                f,
                "Failed to acquire lock on application state. The app may be in an inconsistent state."
            ),
            Error::Json(msg) => write!(f, "JSON Serialization Error: {}", msg),
            Error::GameNotStarted => write!(f, "Game is not started yet"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::Game(_) => StatusCode::BAD_REQUEST,
            Error::MutexPoisoned => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GameNotStarted => StatusCode::CONFLICT,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}

// Implementation of the standard `Error` trait.
impl std::error::Error for Error {}

// `From` implementation to convert Mutex poison errors into our custom `Error`.
// This allows us to use `?` on `mutex.lock()`.
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::MutexPoisoned
    }
}

// `From` implementation to convert `serde_json` errors.
// This allows us to use `?` on serialization/deserialization functions.
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

// `From` implementation to easily convert string slices into a Game error.
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Game(s.to_string())
    }
}

/// A type alias for `std::result::Result` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
