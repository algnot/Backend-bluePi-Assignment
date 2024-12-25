use actix_web::{FromRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde::{Serialize};
use crate::common::jwt::{convert_token_type, create_jwt, TokenType};
use crate::common::request::{AuthTokenHeader, ErrorResponse};
use crate::repository::auth_token::AuthToken;

#[derive(Serialize)]
struct AccessTokenResponse {
    access_token: String
}

pub async fn generate_access_token(auth: AuthTokenHeader) -> impl Responder {
    if convert_token_type(&auth.token.token_type) != convert_token_type(&TokenType::RefreshToken) {
        return HttpResponse::Unauthorized().json( ErrorResponse {
            is_error: true,
            message: "token is invalid".to_string()
        });
    }

    let user = auth.user.unwrap();

    let now = Utc::now();
    let mut access_token = AuthToken::new(&TokenType::AccessToken, &user.id);
    let exp_access_token = now.checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp();
    access_token.uid = user.id;
    access_token.iat = now.timestamp() as i32;
    access_token.exp = exp_access_token as i32;
    let access_token_sting = create_jwt(&access_token.uid,
                                        &access_token.id,
                                        TokenType::AccessToken,
                                        access_token.iat as usize,
                                        access_token.exp as usize).unwrap_or_else(|e| e.to_string());
    access_token.save_to_database();


    HttpResponse::Ok().json( AccessTokenResponse {
        access_token: access_token_sting
    })
}
