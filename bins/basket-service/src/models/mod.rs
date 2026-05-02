// models/basket.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketItem {
    pub product_id: i32,
    pub name: String,
    pub price: f64,
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

    pub fn total(&self) -> f64 {
        self.items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum()
    }
}

#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub product_id: i32,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}
