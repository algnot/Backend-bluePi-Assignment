use actix_web::web;
use crate::service::http::order::create_order::create_order;

pub fn order_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/create", web::post().to(create_order));
}
