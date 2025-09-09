use axum::http::StatusCode;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, StatusCode> {
    tracing::debug!("Verifying token: {}", token);
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default()
    ).map_err(|e| {
        tracing::error!("Token verification error: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?;

    tracing::debug!("Token verified successfully for user ID: {}", token_data.claims.sub);
    Ok(token_data.claims)
}

pub async fn extract_user_id(auth: Option<TypedHeader<Authorization<Bearer>>>, secret: &[u8]) -> Result<i64, StatusCode> {
    let auth = auth.ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = verify_token(auth.token(), secret)?;
    let user_id = claims.sub.parse::<i64>().map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(user_id)
}
