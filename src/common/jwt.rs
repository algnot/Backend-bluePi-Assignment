use std::fmt;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::common::config::get_config;
pub enum JWTError {
    JWTTokenCreationError,
    JWTTokenDecodingError,
}

impl fmt::Display for JWTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JWTError::JWTTokenCreationError => write!(f, "JWT token creation failed"),
            JWTError::JWTTokenDecodingError => write!(f, "JWT token decoding failed"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TokenType {
    TokenTypeAccessToken,
    TokenTypeRefreshToken,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenSub {
    token_id: String,
    uid: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: TokenSub,
    token_type: TokenType,
    exp: usize,
    iat: usize,
}

pub fn create_jwt(uid: &str, token_id: &str, token_type: TokenType) -> Result<String, JWTError> {
    let expiration:i64;
    match token_type {
        TokenType::TokenTypeAccessToken => {
            expiration = Utc::now()
                .checked_add_signed(Duration::minutes(15))
                .expect("valid timestamp")
                .timestamp();
        }
        TokenType::TokenTypeRefreshToken => {
            expiration = Utc::now()
                .checked_add_signed(Duration::days(7))
                .expect("valid timestamp")
                .timestamp();
        }
    }

    let claims = Claims {
        sub: TokenSub {
            token_id: token_id.to_string(),
            uid: uid.to_string(),
        },
        token_type,
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(get_config("JWT_SECRET", "secret").as_ref()))
        .map_err(|_| JWTError::JWTTokenCreationError)
}