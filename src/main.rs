use crate::di::server::init_api_server;

pub mod di;
pub mod entity;
pub mod repository;
pub mod schema;
pub mod service;
pub mod common;

fn main() {
    let _ = init_api_server();
}
