use actix_web::{web};
use crate::service::http::users::user_router;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").configure(user_router));
}
