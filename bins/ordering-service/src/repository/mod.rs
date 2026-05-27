use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{
    error::{OrderError, Result},
    models::{Order, OrderItem, OrderStatus, VerifiedItem},
};

#[derive(Clone)]
pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_order(
        &self,
        user_id: i32,
        items: Vec<VerifiedItem>,
        total: Decimal,
    ) -> Result<Order> {
        let mut tx = self.pool.begin().await?;

        let order = sqlx::query_as!(
            Order,
            r#"INSERT INTO orders (user_id, total)
               VALUES ($1, $2)
               RETURNING id, user_id, status AS "status: OrderStatus", total, created_at"#,
            user_id,
            total
        )
        .fetch_one(&mut *tx)
        .await?;

        for item in &items {
            sqlx::query!(
                "INSERT INTO order_items (order_id, product_id, name, price, quantity)
                 VALUES ($1, $2, $3, $4, $5)",
                order.id,
                item.product_id,
                item.name,
                item.price,
                item.quantity
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(order)
    }

    pub async fn get_order_by_id(&self, id: i32) -> Result<Option<Order>> {
        sqlx::query_as!(
            Order,
            r#"SELECT id, user_id, status AS "status: OrderStatus", total, created_at
               FROM orders WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(OrderError::from)
    }

    pub async fn get_items_by_order_id(&self, order_id: i32) -> Result<Vec<OrderItem>> {
        sqlx::query_as!(
            OrderItem,
            "SELECT * FROM order_items WHERE order_id = $1 ORDER BY id",
            order_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(OrderError::from)
    }

    pub async fn list_by_user_id(&self, user_id: i32) -> Result<Vec<Order>> {
        sqlx::query_as!(
            Order,
            r#"SELECT id, user_id, status AS "status: OrderStatus", total, created_at
               FROM orders WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(OrderError::from)
    }
}
