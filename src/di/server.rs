use actix_web::{middleware, web, App, HttpServer};
use crate::common::config::get_config;
use crate::service::http::router::main_router;

#[actix_web::main]
pub async fn init_api_server() -> std::io::Result<()> {
    let server_port: u16 = get_config("SERVER_PORT", "8080")
        .parse().unwrap_or_else(|_| {
            eprintln!("Invalid SERVER_PORT; defaulting to 8080");
            8080
        });

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("").configure(main_router))
    })
        .bind(("127.0.0.1", server_port))?
        .run()
        .await
}
