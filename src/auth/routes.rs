use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use std::env;
use crate::auth::{Claims, JwtConfig};
use crate::models::user::User;
use bcrypt::verify;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    expires_in: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[post("/authenticate")]
pub async fn authenticate(
    auth: web::Json<AuthRequest>,
    jwt_config: web::Data<JwtConfig>,
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    match User::find_by_username(&auth.username, &pool).await {
        Ok(user) => {
            if verify(&auth.password, &user.password_hash).unwrap_or(false) {
                let expiry_hours: i64 = env::var("JWT_EXPIRY_HOURS")
                    .unwrap_or("24".to_string())
                    .parse()
                    .unwrap_or(24);

                let expiry = Utc::now() + Duration::hours(expiry_hours);
                
                let claims = Claims {
                    sub: auth.username.clone(),
                    exp: expiry.timestamp() as usize,
                    iat: Utc::now().timestamp() as usize, // issued at
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &jwt_config.encoding_key,
                ).unwrap();

                HttpResponse::Ok().json(AuthResponse {
                    token,
                    expires_in: expiry_hours * 3600,
                })
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}