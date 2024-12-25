use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::encryptor::{decrypt, verify_password};
use crate::common::jwt::TokenType;
use crate::common::request::{convert_validate_error_to_response, ErrorResponse};
use crate::repository::auth_token::AuthToken;
use crate::repository::users::User;

#[derive(Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    id: String,
    username: String,
    email: String,
    access_token: String,
    refresh_token: String
}

pub async fn login_user(payload: web::Json<LoginRequest>) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let user = User::new().find_by_email(&payload.email);

    match user {
        Some(user) => {
            let hash_password = decrypt(&user.hashed_password);

            if !verify_password(&payload.password, &hash_password).unwrap() {
                return HttpResponse::BadRequest().json( ErrorResponse{
                    is_error: true,
                    message: format!("password is not correct for {}", payload.email).to_string(),
                })
            }

            let (access_token, refresh_token) = AuthToken::new(&TokenType::AccessToken, &user.id).generate_token();

            HttpResponse::BadRequest().json( LoginResponse{
                id: user.id,
                username: decrypt(&user.username),
                email: decrypt(&user.email),
                access_token,
                refresh_token,
            })
        }
        None => {
            HttpResponse::BadRequest().json( ErrorResponse{
                is_error: true,
                message: format!("cannot find auth with email {}", payload.email).to_string(),
            })
        }
    }
}
