use actix_web::web;
use crate::service::http::product_type::create::create_product_type;

pub fn product_type_router(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(create_product_type));
}
