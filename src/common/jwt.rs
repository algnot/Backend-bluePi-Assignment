use std::fmt;
use std::io::Write;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::warn;
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
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    AccessToken,
    RefreshToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub token_id: String,
    pub uid: String,
    pub token_type: TokenType,
    pub(crate) exp: usize,
    pub iat: usize,
}

pub fn convert_token_type(token_type: &TokenType) -> i32 {
    match token_type {
        TokenType::AccessToken => 0,
        TokenType::RefreshToken => 1,
    }
}

pub fn create_jwt(uid: &String, token_id: &String, token_type: TokenType, iat: usize, exp: usize) -> Result<String, JWTError> {
    let claims = Claims {
        sub: format!("{}.{}", token_id, uid),
        token_id: token_id.to_string(),
        uid: uid.to_string(),
        token_type,
        exp,
        iat,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_config("JWT_SECRET", "ThIsIsAsEcReT").as_ref())).map_err(|_| JWTError::JWTTokenCreationError)
}

pub fn decode_jwt(token: &String) -> Result<Claims, JWTError> {
    let secret = get_config("JWT_SECRET", "ThIsIsAsEcReT");
    let result = decode(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

    match result {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            warn!("Failed to decode token: {}", e);
            Err(JWTError::JWTTokenDecodingError)
        }
    }
}
