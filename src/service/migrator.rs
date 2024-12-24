use log::{info};
use crate::common::config::get_config;
use crate::repository::users::User;

pub fn migrate_data() {
    info!("Migrating data...");
    let with_migrate: String = get_config(&"SERVER_WITH_MIGRATE", &"false");

    if with_migrate != "true" {
        return;
    }

    let user = User::new();
    user.create(&get_config("SERVER_ROOT_EMAIL", "admin@admin.com"),
                &get_config("SERVER_ROOT_USER", "admin@admin.com"),
                &get_config("SERVER_ROOT_PASSWORD", "password"));
}
