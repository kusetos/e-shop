use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Request, State},
    middleware,
    response::Response,
    routing::any,
    Extension, Router,
};
use reqwest::Client;
use tower_http::cors::CorsLayer;

mod auth;
mod config;
mod proxy;

use auth::Claims;
use config::Config;

pub struct AppState {
    pub config: Config,
    pub client: Client,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        config: Config::from_env(),
        client: Client::new(),
    });

    // Public routes — no JWT required
    let public = Router::new()
        .route("/api/auth/register",    any(public_handler))
        .route("/api/auth/login",       any(public_handler))
        .route("/api/products",         any(public_handler))
        .route("/api/products/*rest", any(public_handler))
        .route("/api/categories",       any(public_handler))
        .route("/api/categories/*rest", any(public_handler));

    // Protected routes — JWT required, Claims injected by middleware
    let protected = Router::new()
        .route("/api/auth/me",           any(protected_handler))
        .route("/api/basket/*rest",    any(protected_handler))
        .route("/api/orders",            any(protected_handler))
        .route("/api/orders/*rest",    any(protected_handler))
        .layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware));

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Gateway running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn public_handler(State(state): State<Arc<AppState>>, req: Request) -> Response {
    let target = resolve_target(&state.config, req.uri().path());
    proxy::forward(&state.client, target, req, None).await
}

async fn protected_handler(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    req: Request,
) -> Response {
    let target = resolve_target(&state.config, req.uri().path());
    proxy::forward(&state.client, target, req, Some(&claims)).await
}

fn resolve_target<'a>(config: &'a Config, path: &str) -> &'a str {
    if path.starts_with("/api/auth") {
        &config.identity_url
    } else if path.starts_with("/api/products") || path.starts_with("/api/categories") {
        &config.catalog_url
    } else if path.starts_with("/api/basket") {
        &config.basket_url
    } else if path.starts_with("/api/orders") {
        &config.ordering_url
    } else {
        &config.catalog_url
    }
}
