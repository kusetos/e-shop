use axum::Router;
use std::{net::SocketAddr, sync::Arc};

use crate::{
    db::postgres::create_pool,
    handlers::{categories::categories_router, products::products_router},
    repository::{category::CategoryRepository, product::ProductRepository},
};

mod db;
mod handlers;
mod models;
mod repository;
#[derive(Clone)]
struct AppState {
    product_repo: ProductRepository,
    category_repo: CategoryRepository,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = create_pool().await;
    let product_repo = ProductRepository::new(pool.clone());
    let category_repo = CategoryRepository::new(pool);
    let app_state = Arc::new(AppState {
        product_repo,
        category_repo,
    });
    let app = Router::new().nest(
        "/api",
        products_router(app_state.clone()).merge(categories_router(app_state.clone())),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
