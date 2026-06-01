use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use crate::{
    AppState,
    error::AuthError,
    jwt::{Claims, encode_jwt},
    models::{AuthResponse, LoginRequest, RegisterRequest, UserProfile},
};

pub fn auth_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
        .with_state(state)
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AuthError> {
    if state.user_repo.find_by_email(&req.email).await?.is_some() {
        return Err(AuthError::EmailAlreadyExists);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AuthError::TokenCreation)?
        .to_string();

    let user = state.user_repo.create(&req.email, &hash).await?;

    let token = encode_jwt(user.id, &user.email, &state.jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user_id: user.id,
            email: user.email,
        }),
    ))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    let user = state
        .user_repo
        .find_by_email(&req.email)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| AuthError::TokenCreation)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::InvalidCredentials)?;

    let token = encode_jwt(user.id, &user.email, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    }))
}

async fn me(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserProfile>, AuthError> {
    let user = state
        .user_repo
        .find_by_id(claims.sub)
        .await?
        .ok_or(AuthError::NotFound)?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        created_at: user.created_at,
    }))
}
