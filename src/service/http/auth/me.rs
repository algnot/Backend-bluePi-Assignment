use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use crate::common::encryptor::decrypt;
use crate::common::jwt::{convert_token_type, TokenType};
use crate::common::request::{AuthTokenHeader, ErrorResponse};

#[derive(Serialize)]
struct GetUserInfoResponse {
    id: String,
    username: String,
    email: String,
    active: bool,
}

pub async fn get_user_info(auth: AuthTokenHeader) -> impl Responder {
    if convert_token_type(&auth.token.token_type) != convert_token_type(&TokenType::AccessToken) {
        return HttpResponse::Unauthorized().json( ErrorResponse {
            is_error: true,
            message: "token is invalid".to_string()
        });
    }

    let user = auth.user.unwrap();

    HttpResponse::Ok().json( GetUserInfoResponse {
        id: user.id,
        username: decrypt(&user.username),
        email: decrypt(&user.email),
        active: user.active.unwrap(),
    })
}
