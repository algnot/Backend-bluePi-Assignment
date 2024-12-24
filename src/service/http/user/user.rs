use actix_web::{web};
use crate::service::http::user::register::register_user;

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user));
}


