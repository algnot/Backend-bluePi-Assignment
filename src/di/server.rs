use std::fmt::format;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use crate::common::config::get_config;
use crate::service::http::router::main_router;

#[actix_web::main]
pub async fn init_api_server() -> std::io::Result<()> {
    let server_port: u16 = get_config("SERVER_PORT", "9000")
        .parse().unwrap_or_else(|_| {
        eprintln!("Invalid SERVER_PORT; defaulting to 9000");
        9000
    });

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::scope("").configure(main_router))
    })
        .bind(format!("0.0.0.0:{}", server_port))?
        .run()
        .await
}
