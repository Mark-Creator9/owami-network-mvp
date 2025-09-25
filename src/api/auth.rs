use axum::{extract::State, Json, http::StatusCode, async_trait, extract::FromRequestParts, http::request::Parts, response::{Response, IntoResponse}};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::postgres::Postgres;
use uuid::Uuid;
use crate::models::user::User;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let hashed_password = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to process password" })),
            );
        }
    };

    let user_id = Uuid::new_v4().to_string();

    let result = sqlx::query!(
        "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)",
        user_id,
        payload.username,
        hashed_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let jwt_secret = match std::env::var("JWT_SECRET") {
                Ok(secret) => secret,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": "JWT_SECRET environment variable not set" })),
                    );
                }
            };
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims { sub: user_id, exp: expiration as usize };
            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())) {
                Ok(t) => t,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": "Failed to create token" })),
                    );
                }
            };

            (StatusCode::CREATED, Json(serde_json::json!({ "token": token })))
        },
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT,
                        Json(serde_json::json!({ "error": "Username already exists" })),
                    );
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create user" })),
            )
        }
    }
}

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user = match User::find_by_username(&payload.username, &pool).await {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid credentials" })),
            );
        }
    };

    let valid_password = match bcrypt::verify(&payload.password, &user.password_hash) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to verify password" })),
            );
        }
    };

    if !valid_password {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid credentials" })),
        );
    }

    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "JWT_SECRET environment variable not set" })),
            );
        }
    };
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims { sub: user.id.clone(), exp: expiration as usize };
    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())) {
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
        let auth_header = parts.headers.get("Authorization").and_then(|header| header.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                let jwt_secret = match std::env::var("JWT_SECRET") {
                    Ok(secret) => secret,
                    Err(_) => return Err(AuthError::InvalidToken),
                };
                let validation = Validation::default();

                let token_data = decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &validation)
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

