use actix_web::web;
use crate::service::http::product_type::create_product_type::create_product_type;
use crate::service::http::product_type::get_product_type::get_product_type;
use crate::service::http::product_type::list_product_type::list_product_type;
use crate::service::http::product_type::update_product_type::update_product_type;

pub fn product_type_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/create", web::post().to(create_product_type));
    cfg.route("/list", web::get().to(list_product_type));
    cfg.route("/{id}", web::get().to(get_product_type));
    cfg.route("/update", web::put().to(update_product_type));
}
