use actix_web::{web, HttpResponse, Responder};

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user));
}

async fn register_user() -> impl Responder {
    HttpResponse::Ok().body("Test user")
}
