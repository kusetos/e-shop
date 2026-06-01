// models/basket.rs
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketItem {
    pub product_id: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basket {
    pub user_id: i32,
    pub items: Vec<BasketItem>,
}

impl Basket {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            items: vec![],
        }
    }

    pub fn total(&self) -> Decimal {
        self.items
            .iter()
            .map(|item| item.price * Decimal::from(item.quantity))
            .sum()
    }
}

#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}
