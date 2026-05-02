// repository/basket.rs
use crate::models::{AddItemRequest, Basket};
use redis::AsyncCommands;

pub struct BasketRepository {
    client: redis::Client,
}

impl BasketRepository {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    fn key(user_id: i32) -> String {
        format!("basket:{}", user_id)
    }

    pub async fn get(&self, user_id: i32) -> crate::error::Result<Basket> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = Self::key(user_id);

        let data: Option<String> = conn.get(&key).await?;

        match data {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(Basket::new(user_id)),
        }
    }

    pub async fn add_item(
        &self,
        user_id: i32,
        req: AddItemRequest,
    ) -> crate::error::Result<Basket> {
        let mut basket = self.get(user_id).await?;

        if let Some(item) = basket
            .items
            .iter_mut()
            .find(|i| i.product_id == req.product_id)
        {
            item.quantity += req.quantity;
        } else {
            basket.items.push(crate::models::BasketItem {
                product_id: req.product_id,
                name: req.name,
                price: req.price,
                quantity: req.quantity,
            });
        }

        self.save(&basket).await?;
        Ok(basket)
    }

    pub async fn remove_item(&self, user_id: i32, product_id: i32) -> crate::error::Result<Basket> {
        let mut basket = self.get(user_id).await?;
        basket.items.retain(|i| i.product_id != product_id);
        self.save(&basket).await?;
        Ok(basket)
    }

    pub async fn clear(&self, user_id: i32) -> crate::error::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.del(Self::key(user_id)).await?;
        Ok(())
    }

    async fn save(&self, basket: &Basket) -> crate::error::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let json = serde_json::to_string(basket)?;
        let _: () = conn
            .set_ex(Self::key(basket.user_id), json, 60 * 60 * 24 * 7)
            .await?;
        Ok(())
    }
}
