use actix_web::web;
use crate::service::http::order::create_order::create_order;
use crate::service::http::order::get_order::get_order;

pub fn order_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/create", web::post().to(create_order));
    cfg.route("/{name}", web::get().to(get_order));
}
