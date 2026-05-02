// routes.rs
use crate::{
    AppState,
    handlers::{add_item, clear_basket, get_basket, remove_item},
};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/basket/:user_id", get(get_basket))
        .route("/basket/:user_id", post(add_item))
        .route("/basket/:user_id", delete(clear_basket))
        .route("/basket/:user_id/:product_id", delete(remove_item))
        .with_state(state)
}
