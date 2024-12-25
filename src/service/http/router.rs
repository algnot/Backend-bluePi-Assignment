use actix_web::{web};
use crate::service::http::auth::auth::user_router;
use crate::service::http::product_type::product_type::product_type_router;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(user_router));
    cfg.service(web::scope("/product-type").configure(product_type_router));
}
