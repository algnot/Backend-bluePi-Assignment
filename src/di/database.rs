use diesel::prelude::*;
use dotenv::dotenv;
use crate::common::config::get_config;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = get_config("DATABASE_URL", "");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
