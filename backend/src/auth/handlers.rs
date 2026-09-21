use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    Json,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    auth::jwt::{create_jwt, AuthUser},
    error::AppError,
    AppState,
};

#[derive(Serialize)]
pub struct AuthStatusResponse {
    pub initialized: bool,
}

#[derive(Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

pub async fn auth_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuthStatusResponse>, AppError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?;

    Ok(Json(AuthStatusResponse {
        initialized: row.0 > 0,
    }))
}

pub async fn setup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SetupRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?;

    if row.0 > 0 {
        return Err(AppError::BadRequest("Setup has already been completed".to_string()));
    }

    let username = payload.username.trim();
    if username.is_empty() {
        return Err(AppError::BadRequest("Username cannot be empty".to_string()));
    }

    if payload.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Password must be at least 6 characters long".to_string(),
        ));
    }

    if payload.password != payload.confirm_password {
        return Err(AppError::BadRequest("Passwords do not match".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing error: {}", e)))?
        .to_string();

    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(&password_hash)
        .execute(&state.db)
        .await?;

    let token = create_jwt(username, &state.config.jwt_secret)?;

    let mut headers = HeaderMap::new();
    let cookie_val = format!(
        "token={}; HttpOnly; Path=/; SameSite=Strict; Max-Age=2592000",
        token
    );
    headers.insert(header::SET_COOKIE, HeaderValue::from_str(&cookie_val).unwrap());

    Ok((
        headers,
        Json(AuthResponse {
            token,
            user: UserResponse {
                username: username.to_string(),
            },
        }),
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let row: Option<(i64, String, String)> =
        sqlx::query_as("SELECT id, username, password_hash FROM users WHERE username = ?")
            .bind(&payload.username)
            .fetch_optional(&state.db)
            .await?;

    let (_, username, stored_hash) = row.ok_or_else(|| {
        AppError::Auth("Invalid username or password".to_string())
    })?;

    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Auth("Invalid username or password".to_string()))?;

    let token = create_jwt(&username, &state.config.jwt_secret)?;

    let mut headers = HeaderMap::new();
    let cookie_val = format!(
        "token={}; HttpOnly; Path=/; SameSite=Strict; Max-Age=2592000",
        token
    );
    headers.insert(header::SET_COOKIE, HeaderValue::from_str(&cookie_val).unwrap());

    Ok((
        headers,
        Json(AuthResponse {
            token,
            user: UserResponse { username },
        }),
    ))
}

pub async fn logout() -> Result<(HeaderMap, StatusCode), AppError> {
    let mut headers = HeaderMap::new();
    let cookie_val = "token=; HttpOnly; Path=/; SameSite=Strict; Max-Age=0";
    headers.insert(header::SET_COOKIE, HeaderValue::from_str(cookie_val).unwrap());

    Ok((headers, StatusCode::OK))
}

pub async fn me(auth: AuthUser) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(UserResponse {
        username: auth.username,
    }))
}
