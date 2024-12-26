use actix_web::web;
use crate::service::http::system::get_coin::get_coin;
use crate::service::http::system::insert_coin::insert_coin;

pub fn system_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/get-coin", web::get().to(get_coin));
    cfg.route("/insert-coin", web::post().to(insert_coin));
}
