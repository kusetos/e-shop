use sqlx::PgPool;

use crate::models::products::{CreateProductRequest, Product};

#[derive(Clone)]
pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as!(Product, "SELECT * FROM products ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<Option<Product>, sqlx::Error> {
        sqlx::query_as!(Product, "SELECT * FROM products WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_products_by_category(
        &self,
        category_id: i32,
    ) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as!(
            Product,
            "SELECT * FROM products WHERE category_id = $1 ORDER BY id",
            category_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_product(&self, req: CreateProductRequest) -> Result<Product, sqlx::Error> {
        sqlx::query_as!(
            Product,
            "INSERT INTO products (name, description, price, stock, image_url, category_id) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING *",
            req.name,
            req.description,
            req.price,
            req.stock,
            req.image_url,
            req.category_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_product_by_id(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM products WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update_product_by_id(
        &self,
        id: i32,
        req: CreateProductRequest,
    ) -> Result<Option<Product>, sqlx::Error> {
        sqlx::query_as!(
            Product,
            "UPDATE products
             SET name = $1,
                 description = $2,
                 price = $3,
                 stock = $4,
                 image_url = $5,
                 category_id = $6
             WHERE id = $7
             RETURNING *",
            req.name,
            req.description,
            req.price,
            req.stock,
            req.image_url,
            req.category_id,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
