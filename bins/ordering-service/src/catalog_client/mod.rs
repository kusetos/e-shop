use reqwest::Client;
use serde::Deserialize;

use crate::error::OrderError;

#[derive(Clone)]
pub struct CatalogClient {
    base_url: String,
    client: Client,
}

// Only the fields we need from catalog-service's product response
#[derive(Debug, Deserialize)]
pub struct CatalogProduct {
    pub id: i32,
    pub name: String,
    pub price: rust_decimal::Decimal,
}

impl CatalogClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn get_product(&self, product_id: i32) -> Result<CatalogProduct, OrderError> {
        let url = format!("{}/api/products/{}", self.base_url, product_id);
        let response = self.client.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(OrderError::ProductNotFound(product_id));
        }

        let product = response.error_for_status()?.json::<CatalogProduct>().await?;
        Ok(product)
    }
}
