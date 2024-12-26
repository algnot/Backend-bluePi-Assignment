use actix_web::{web};
use crate::service::http::auth::auth::user_router;
use crate::service::http::order::order::order_router;
use crate::service::http::product::product::product_router;
use crate::service::http::product_type::product_type::product_type_router;
use crate::service::http::system::system::system_router;
use crate::service::http::uploader::uploader::uploader_router;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(user_router));
    cfg.service(web::scope("/product-type").configure(product_type_router));
    cfg.service(web::scope("/uploader").configure(uploader_router));
    cfg.service(web::scope("/product").configure(product_router));
    cfg.service(web::scope("/system").configure(system_router));
    cfg.service(web::scope("/order").configure(order_router));
}
