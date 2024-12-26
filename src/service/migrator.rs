use log::{info};
use crate::common::config::get_config;
use crate::repository::system_parameter::SystemParameter;
use crate::repository::users::User;

pub fn migrate_data() {
    info!("Migrating data...");
    let with_migrate: String = get_config(&"SERVER_WITH_MIGRATE", &"false");

    if with_migrate != "true" {
        return;
    }

    User::new().create(&get_config("SERVER_ROOT_EMAIL", "admin@admin.com"),
                &get_config("SERVER_ROOT_USER", "admin@admin.com"),
                &get_config("SERVER_ROOT_PASSWORD", "password"));

    SystemParameter::new().create_or_update(&"coin-1".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"coin-5".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"coin-10".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"bank-20".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"bank-50".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"bank-100".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"bank-500".to_string(), &"0".to_string(), true);
    SystemParameter::new().create_or_update(&"bank-1000".to_string(), &"0".to_string(), true);

    info!("Successfully migrated data!");
}
