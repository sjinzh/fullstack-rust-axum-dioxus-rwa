use crate::app_errors::Result;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: i64) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret("TODO_JWT_SECRET_AS_CONFIG".as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret("TODO_JWT_SECRET_AS_CONFIG".as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
