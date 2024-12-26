use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::di::database::establish_connection;
use log::warn;
use diesel::prelude::*;
use crate::schema::{system_parameter};
use crate::schema::system_parameter::key_name;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::system_parameter)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SystemParameter {
    pub id: i32,
    pub key_name: String,
    pub key_value: String,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::system_parameter)]
pub struct NewSystemParameter {
    pub key_name: String,
    pub key_value: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl SystemParameter {
    pub fn new() -> Self {
        Self {
            id: -1,
            key_name: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
            key_value: "".to_string(),
        }
    }

    pub fn get_by_key_name(&self, key: &String) -> Option<SystemParameter> {
        let conn = &mut establish_connection();

        let result = system_parameter::table
            .filter(key_name.eq(key))
            .select(SystemParameter::as_select())
            .first(conn)
            .optional();

        match result {
            Ok(Some(system_parameter)) => Some(system_parameter),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find product type with error: {}", e);
                None
            }
        }
    }

    pub fn update_by_key_value(&self, key: &String, value: &String) -> Option<SystemParameter> {
        let conn = &mut establish_connection();

        let update_result = diesel::update(system_parameter::table.filter(key_name.eq(key)))
            .set((
                system_parameter::key_value.eq(value),
            ))
            .execute(conn);

        match update_result {
            Ok(rows_updated) if rows_updated > 0 => {
                self.get_by_key_name(key)
            }
            Ok(_) => {
                warn!("No system param found with key name: {}", key);
                None
            }
            Err(e) => {
                warn!("Failed to system param type with key name: {} {}", key, e);
                None
            }
        }
    }

    pub fn create_or_update(&self, key: &String, value: &String, skip_update: bool) -> Option<SystemParameter> {
        let conn = &mut establish_connection();

        let existing_key = self.get_by_key_name(&key);

        match existing_key {
            Some(existing) => {
                if skip_update {
                    return Option::from(existing)
                }
                self.update_by_key_value(&key, &value)
            }
            None => {
                diesel::insert_into(system_parameter::table)
                    .values( NewSystemParameter {
                        key_name: key.clone(),
                        key_value: value.clone(),
                        created_at: self.created_at.clone(),
                        updated_at: self.updated_at.clone(),
                    })
                    .execute(conn)
                    .expect("Error saving new system parameter");

                self.get_by_key_name(key)
            }
        }
    }
}
