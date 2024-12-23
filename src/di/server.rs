use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use crate::common::config::get_config;
use crate::service::http::router::main_router;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
pub async fn init_api_server() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_port: u16 = get_config("SERVER_PORT", "8080")
        .parse().unwrap_or_else(|_| {
            eprintln!("Invalid SERVER_PORT; defaulting to 8080");
            8080
        });

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("").configure(main_router))
            .route("/_hc", web::get().to(health_check))
    })
        .bind(("127.0.0.1", server_port))?
        .run()
        .await
}
