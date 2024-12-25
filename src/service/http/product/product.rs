use actix_web::web;
use crate::service::http::product::create_product::create_product;
use crate::service::http::product::get_product::get_product;
use crate::service::http::product::list_product::list_product;
use crate::service::http::product::update_product::update_product;

pub fn product_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/create", web::post().to(create_product));
    cfg.route("/update", web::put().to(update_product));
    cfg.route("/list", web::get().to(list_product));
    cfg.route("/{id}", web::get().to(get_product));
}
