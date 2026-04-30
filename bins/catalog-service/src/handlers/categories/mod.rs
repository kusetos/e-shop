use std::sync::Arc;

use crate::{
    AppState,
    models::products::{Category, CreateCategoryRequest},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};

pub fn categories_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .with_state(state)
}

async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    state
        .category_repo
        .get_all_categories()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, StatusCode> {
    state
        .category_repo
        .create_category(req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
