use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use rust_decimal::Decimal;

use crate::{
    error::OrderError,
    kafka::OrderCreatedEvent,
    models::{CreateOrderRequest, Order, OrderResponse, UpdateStatusRequest, VerifiedItem},
    AppState,
};

pub fn orders_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/orders", post(create_order))
        .route("/orders/user/:user_id", get(list_user_orders))
        .route("/orders/:id", get(get_order))
        .route("/orders/:id/status", put(update_status))
        .with_state(state)
}

async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), StatusCode> {
    if req.items.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    if req.items.iter().any(|item| item.quantity <= 0) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut verified: Vec<VerifiedItem> = Vec::with_capacity(req.items.len());
    for item in &req.items {
        let product = state
            .catalog_client
            .get_product(item.product_id)
            .await
            .map_err(|e| match e {
                OrderError::ProductNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::BAD_GATEWAY,
            })?;

        let price = product.price.round_dp(2);

        verified.push(VerifiedItem {
            product_id: product.id,
            name: product.name,
            price,
            quantity: item.quantity,
        });
    }

    let total: Decimal = verified
        .iter()
        .map(|i| i.price * Decimal::from(i.quantity))
        .sum();

    let order = state
        .order_repo
        .create_order(req.user_id, verified, total)
        .await
        .map_err(|e| {
            tracing::error!("create_order db error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let items = state
        .order_repo
        .get_items_by_order_id(order.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    state.producer.order_created(&OrderCreatedEvent {
        order_id: order.id,
        user_id:  order.user_id,
    }).await;

    Ok((StatusCode::CREATED, Json(OrderResponse { order, items })))
}

async fn get_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<OrderResponse>, StatusCode> {
    let order = state
        .order_repo
        .get_order_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let items = state
        .order_repo
        .get_items_by_order_id(order.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OrderResponse { order, items }))
}

async fn list_user_orders(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    state
        .order_repo
        .list_by_user_id(user_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("list_user_orders error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn update_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<Order>, StatusCode> {
    state
        .order_repo
        .update_status(id, req.status)
        .await
        .map(Json)
        .map_err(|e| match e {
            OrderError::NotFound => StatusCode::NOT_FOUND,
            OrderError::InvalidStatusTransition => StatusCode::UNPROCESSABLE_ENTITY,
            _ => {
                tracing::error!("update_status error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}
