fn main() {
    println!("Hello, world!");
}
use axum::{extract::Path, http::StatusCode, response::IntoResponse};

// GET  /basket/:user_id        — получить корзину пользователя
pub async fn get_basket(Path(user_id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("get basket for {}", user_id))
}

// POST /basket/:user_id        — добавить товар в корзину
pub async fn add_to_basket(Path(user_id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("add to basket for {}", user_id))
}

// DELETE /basket/:user_id      — очистить корзину
pub async fn clear_basket(Path(user_id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("clear basket for {}", user_id))
}

// DELETE /basket/:user_id/:product_id  — удалить один товар
pub async fn remove_product(
    Path((user_id, product_id)): Path<(String, String)>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("remove product {} from basket {}", product_id, user_id),
    )
}
