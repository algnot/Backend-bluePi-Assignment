use crate::di::server::init_api_server;
use crate::service::migrator::migrate_data;

pub mod di;
pub mod repository;
pub mod schema;
pub mod service;
pub mod common;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    migrate_data();
    let _ = init_api_server();
}
