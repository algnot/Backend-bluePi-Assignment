use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::encryptor::decrypt;
use crate::common::jwt::{TokenType};
use crate::common::request::{convert_validate_error_to_response, ErrorResponse};
use crate::repository::auth_token::AuthToken;
use crate::repository::users::User;

#[derive(Validate, Deserialize)]
pub struct RegisterUserRequest {
    #[validate(length(min = 3))]
    username: String,

    #[validate(length(min = 8))]
    password: String,

    #[validate(email)]
    email: String,
}

#[derive(Serialize)]
struct RegisterUserResponse {
    id: String,
    username: String,
    email: String,
    access_token: String,
    refresh_token: String
}

pub async fn register_user(payload: web::Json<RegisterUserRequest>) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let (user, is_already_exists) = User::new().create(&payload.email, &payload.username, &payload.password);

    if is_already_exists {
        return  HttpResponse::BadRequest().json(ErrorResponse{
            is_error: true,
            message: format!("User with email {} already exists.", payload.email),
        })
    }

    let created_user = user.unwrap();
    let (access_token, refresh_token) = AuthToken::new(&TokenType::AccessToken, &created_user.id).generate_token();

    HttpResponse::Ok().json(RegisterUserResponse {
        id: created_user.id,
        username: decrypt(&created_user.username),
        email: decrypt(&created_user.email),
        access_token,
        refresh_token
    })
}
