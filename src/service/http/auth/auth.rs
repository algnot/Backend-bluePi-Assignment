use actix_web::{web};
use crate::service::http::auth::access_token::generate_access_token;
use crate::service::http::auth::login::login_user;
use crate::service::http::auth::me::get_user_info;
use crate::service::http::auth::register::register_user;

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user));
    cfg.route("/login", web::post().to(login_user));
    cfg.route("/access-token", web::post().to(generate_access_token));
    cfg.route("/me", web::get().to(get_user_info));
}
