use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "order_status", rename_all = "PascalCase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub status: OrderStatus,
    pub total: Decimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
}

// What the client sends — product IDs and quantities only, no prices
#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i32,
    pub items: Vec<OrderItemRequest>,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: OrderStatus,
}

// Internal: after price verification against catalog-service
pub struct VerifiedItem {
    pub product_id: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}
