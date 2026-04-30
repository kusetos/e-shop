use sqlx::PgPool;

use crate::models::products::{Category, CreateCategoryRequest};

#[derive(Clone)]
pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as!(Category, "SELECT * FROM categories ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_category(&self, req: CreateCategoryRequest) -> Result<Category, sqlx::Error> {
        sqlx::query_as!(
            Category,
            "INSERT INTO categories (name) VALUES ($1) RETURNING *",
            req.name
        )
        .fetch_one(&self.pool)
        .await
    }
}
