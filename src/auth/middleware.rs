use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{DecodingKey, Validation};
use crate::auth::{Claims, JwtConfig};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_config = req
        .app_data::<JwtConfig>()
        .expect("JWT config not configured");

    match validate_token(credentials.token(), &jwt_config.decoding_key) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => Err((ErrorUnauthorized(e), req)),
    }
}

fn validate_token(token: &str, key: &DecodingKey) -> Result<Claims, String> {
    jsonwebtoken::decode::<Claims>(
        token,
        key,
        &Validation::default(), // Use default validation which includes HS256
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid token: {}", e))
}