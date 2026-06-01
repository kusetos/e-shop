use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Email already registered")]
    EmailAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token creation failed")]
    TokenCreation,

    #[error("User not found")]
    NotFound,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::Database(_) | AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::EmailAlreadyExists => StatusCode::CONFLICT,
            AuthError::InvalidCredentials | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::NotFound => StatusCode::NOT_FOUND,
        };
        (status, self.to_string()).into_response()
    }
}
