use std::sync::Arc;

use crate::{
    AppState,
    models::products::{CreateProductRequest, Product},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProductFilter {
    category_id: Option<i32>,
}

pub fn products_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/products", get(list_products).post(create_product))
        .route(
            "/products/:id",
            get(get_product).put(update_product).delete(delete_product),
        )
        .with_state(state)
}

async fn list_products(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<ProductFilter>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let result = match filter.category_id {
        Some(cat_id) => state.product_repo.get_products_by_category(cat_id).await,
        None => state.product_repo.get_all_products().await,
    };
    result
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, StatusCode> {
    state
        .product_repo
        .get_product_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    state
        .product_repo
        .create_product(req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    state
        .product_repo
        .update_product_by_id(id, req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_product(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> StatusCode {
    match state.product_repo.delete_product_by_id(id).await {
        Ok(0) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
