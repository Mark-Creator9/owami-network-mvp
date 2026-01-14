use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
// Remove the invalid import
// use crate::db::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn register(
    // Remove the State parameter since we don't have a User model
    Json(_payload): Json<RegisterUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user_id = Uuid::new_v4().to_string();

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "JWT_SECRET environment variable not set" })),
            );
        }
    };
    let expiration = Utc::now()
        .checked_add_signed(Duration::try_hours(24).expect("valid duration"))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create token" })),
            );
        }
    };

    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "token": token })),
    )
}

pub async fn login(Json(_payload): Json<LoginUser>) -> (StatusCode, Json<serde_json::Value>) {
    let user_id = Uuid::new_v4().to_string();

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "JWT_SECRET environment variable not set" })),
            );
        }
    };
    let expiration = Utc::now()
        .checked_add_signed(Duration::try_hours(24).expect("valid duration"))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create token" })),
            );
        }
    };

    (StatusCode::OK, Json(serde_json::json!({ "token": token })))
}

// Middleware
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                let jwt_secret = match env::var("JWT_SECRET") {
                    Ok(secret) => secret,
                    Err(_) => return Err(AuthError::InvalidToken),
                };
                let validation = Validation::default();

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &validation,
                )
                .map_err(|_| AuthError::InvalidToken)?;

                return Ok(token_data.claims);
            }
        }

        Err(AuthError::MissingToken)
    }
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
        };
        let body = Json(serde_json::json!({ "error": error_message }));
        (status, body).into_response()
    }
}

pub async fn profile(claims: Claims) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "user_id": claims.sub
        })),
    )
}
