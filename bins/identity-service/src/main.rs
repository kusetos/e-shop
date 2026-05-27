use std::{net::SocketAddr, sync::Arc};

use axum::Router;

mod db;
mod error;
mod handlers;
mod jwt;
mod models;
mod repository;

use repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: UserRepository,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::create_pool().await;

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let state = Arc::new(AppState {
        user_repo: UserRepository::new(pool),
        jwt_secret,
    });

    let app = Router::new().nest("/api", handlers::auth_router(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    tracing::info!("Identity service running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
