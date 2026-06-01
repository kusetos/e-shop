use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrderError>;

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Product {0} not found in catalog")]
    ProductNotFound(i32),

    #[error("Order not found")]
    NotFound,

    #[error("Invalid status transition")]
    InvalidStatusTransition,
}
