// handlers/basket.rs
use crate::{AppState, models::AddItemRequest};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

// GET /basket/:user_id
pub async fn get_basket(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Get Basket");
    match state.basket_repo.get(user_id).await {
        Ok(basket) => Ok(Json(serde_json::json!({
            "user_id": basket.user_id,
            "items": basket.items,
            "total": basket.total()
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /basket/:user_id
pub async fn add_item(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(req): Json<AddItemRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.basket_repo.add_item(user_id, req).await {
        Ok(basket) => Ok(Json(serde_json::json!({
            "user_id": basket.user_id,
            "items": basket.items,
            "total": basket.total()
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /basket/:user_id
pub async fn clear_basket(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match state.basket_repo.clear(user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /basket/:user_id/:product_id
pub async fn remove_item(
    State(state): State<AppState>,
    Path((user_id, product_id)): Path<(i32, i32)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.basket_repo.remove_item(user_id, product_id).await {
        Ok(basket) => Ok(Json(serde_json::json!({
            "user_id": basket.user_id,
            "items": basket.items,
            "total": basket.total()
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
