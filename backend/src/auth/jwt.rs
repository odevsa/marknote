use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::AppError;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub username: String,
}

pub fn create_jwt(username: &str, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Auth("Invalid or expired authentication token".to_string()))?;

    Ok(token_data.claims)
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_token(parts).ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": "Missing authentication token" })),
            )
        })?;

        let claims = verify_jwt(&token, &state.config.jwt_secret).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": e.to_string() })),
            )
        })?;

        Ok(AuthUser {
            username: claims.sub,
        })
    }
}

fn extract_token(parts: &Parts) -> Option<String> {
    // 1. Check Authorization: Bearer <token>
    if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.trim().to_string());
            }
        }
    }

    // 2. Check Cookie: token=<token>
    if let Some(cookie_header) = parts.headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie_pair in cookie_str.split(';') {
                let mut parts = cookie_pair.trim().splitn(2, '=');
                if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                    if name == "token" {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_creation_and_verification() {
        let secret = "super_secret_test_key_for_jwt_32_bytes";
        let username = "admin";

        let token = create_jwt(username, secret).expect("Token should be created");
        assert!(!token.is_empty());

        let claims = verify_jwt(&token, secret).expect("Token should be verified");
        assert_eq!(claims.sub, username);

        // Verify failure with incorrect secret
        let wrong_secret = "invalid_secret_key_that_does_not_match";
        assert!(verify_jwt(&token, wrong_secret).is_err());
    }
}
