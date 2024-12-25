use actix_web::web;
use crate::service::http::product::create_product::create_product;

pub fn product_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/create", web::post().to(create_product));
}
