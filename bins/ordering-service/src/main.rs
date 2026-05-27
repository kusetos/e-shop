use std::{net::SocketAddr, sync::Arc};

use axum::Router;

mod catalog_client;
mod db;
mod error;
mod handlers;
mod models;
mod repository;

use catalog_client::CatalogClient;
use repository::OrderRepository;

#[derive(Clone)]
pub struct AppState {
    pub order_repo: OrderRepository,
    pub catalog_client: CatalogClient,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::create_pool().await;

    let catalog_url =
        std::env::var("CATALOG_SERVICE_URL").expect("CATALOG_SERVICE_URL must be set");

    let state = Arc::new(AppState {
        order_repo: OrderRepository::new(pool),
        catalog_client: CatalogClient::new(catalog_url),
    });

    let app = Router::new().nest("/api", handlers::orders_router(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    tracing::info!("Ordering service running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
