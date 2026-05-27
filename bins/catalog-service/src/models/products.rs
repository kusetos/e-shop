use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub image_url: String,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(FromRow, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
    pub description: String,
    pub image_url: String,
    pub category_id: i32,
}
