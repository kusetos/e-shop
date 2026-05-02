use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BasketError>;

#[derive(Error, Debug)]
pub enum BasketError {
    #[error("Redis database error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Resource not found")]
    NotFound,

    #[error("Internal server error")]
    Other(#[from] io::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Missing expected field: {0}")]
    MissingField(String),
}
