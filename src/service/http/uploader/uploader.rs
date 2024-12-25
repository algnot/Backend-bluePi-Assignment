use actix_web::web;
use crate::service::http::uploader::get_image::get_image;
use crate::service::http::uploader::upload_image::upload_image;

pub fn uploader_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/image", web::post().to(upload_image));
    cfg.route("/image/{id}", web::get().to(get_image));
}
