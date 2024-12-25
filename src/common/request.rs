use std::future::{ready, Ready};
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use chrono::Utc;
use serde::Serialize;
use validator::{ValidationErrors};
use crate::common::jwt::{decode_jwt, Claims};
use crate::repository::auth_token::AuthToken;
use crate::repository::users::User;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) is_error: bool,
    pub(crate) message: String,
}

pub fn convert_validate_error_to_response(e: ValidationErrors) -> ErrorResponse {
    let missing_fields = e.field_errors();
    let mut message = "validate error in field: ".to_string();

    for (field, _error) in missing_fields {
        message.push_str(format!("`{}` ", field).as_str());
    }

    ErrorResponse {
        is_error: true,
        message,
    }
}

#[derive(Debug, Serialize)]
pub struct AuthTokenHeader {
    pub token: Claims,
    pub user: Option<User>,
}

impl FromRequest for AuthTokenHeader {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").to_string();
                    let claims = decode_jwt(&token);

                    return match claims {
                        Ok(claims) => {
                            let now = Utc::now().timestamp() as usize;
                            if now > claims.exp {
                                return ready(Err(actix_web::error::ErrorUnauthorized("Token has expired")));
                            }

                            let auth_token = AuthToken::new(&claims.token_type, &claims.uid).get_token_info(&claims.token_id).unwrap();

                            if auth_token.is_revoke.unwrap() {
                                return ready(Err(actix_web::error::ErrorUnauthorized("Token is revoked")));
                            }

                            let user = User::new().find_by_id(&claims.uid).unwrap();

                            ready(Ok(AuthTokenHeader { token: claims , user: Some(user) }))
                        }
                        Err(err) => {
                            ready(Err(actix_web::error::ErrorUnauthorized(err.to_string())))
                        }
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header")))
    }
}

