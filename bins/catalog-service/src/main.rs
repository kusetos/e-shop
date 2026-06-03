use axum::Router;
use std::{net::SocketAddr, sync::Arc};

use crate::{
    db::postgres::create_pool,
    handlers::{categories::categories_router, products::products_router},
    repository::{category::CategoryRepository, product::ProductRepository},
};

mod db;
mod handlers;
mod kafka;
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
    tracing_subscriber::fmt::init();

    let kafka_brokers = std::env::var("KAFKA_BROKERS")
        .expect("KAFKA_BROKERS must be set");

    let pool = create_pool().await;
    let product_repo = ProductRepository::new(pool.clone());
    let category_repo = CategoryRepository::new(pool);

    tokio::spawn(kafka::start_consumer(kafka_brokers, product_repo.clone()));

    let app_state = Arc::new(AppState {
        product_repo,
        category_repo,
    });
    let app = Router::new().nest(
        "/api",
        products_router(app_state.clone()).merge(categories_router(app_state.clone())),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Catalog service running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
