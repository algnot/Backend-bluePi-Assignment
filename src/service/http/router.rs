use actix_web::{web};
use crate::service::http::auth::user::user_router;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(user_router));
}
