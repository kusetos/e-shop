use std::sync::Arc;

use crate::repository::BasketRepository;

mod error;
mod handlers;
mod models;
mod repository;
mod routes;
#[derive(Clone)]
pub struct AppState {
    pub basket_repo: Arc<BasketRepository>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap();

    let client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    let basket_repo = Arc::new(BasketRepository::new(client));
    let state = AppState {
        basket_repo: basket_repo,
    };

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    tracing::info!("🛒 Basket service running on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}
