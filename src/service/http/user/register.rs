use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::jwt::{create_jwt, TokenType};
use crate::common::request::{convert_validate_error_to_response, ErrorResponse};
use crate::repository::users::User;

#[derive(Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 3))]
    username: String,

    #[validate(length(min = 8))]
    password: String,

    #[validate(email)]
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    access_token: String,
    refresh_token: String
}

pub async fn register_user(payload: web::Json<RegisterUser>) -> impl Responder {
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

    let access_token = create_jwt(&created_user.id, "TokenTypeAccessToken id", TokenType::TokenTypeAccessToken).unwrap_or_else(|e| e.to_string());
    let refresh_token = create_jwt(&created_user.id, "TokenTypeRefreshToken id", TokenType::TokenTypeRefreshToken).unwrap_or_else(|e| e.to_string());

    HttpResponse::Ok().json(UserResponse {
        id: created_user.id,
        access_token,
        refresh_token
    })
}
