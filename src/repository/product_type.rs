use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use log::warn;
use serde::Serialize;
use diesel::prelude::*;
use crate::common::request::AuthTokenHeader;
use crate::di::database::establish_connection;
use crate::repository::users::User;
use crate::schema::users;
use crate::schema::product_type;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::product_type)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProductType {
    pub id: String,
    pub name: String,
    pub active: Option<bool>,
    
    // meta data
    pub created_by: String,
    pub updated_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl ProductType {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            active: Some(true),
            created_by: "".to_string(),
            updated_by: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }

    pub fn get_product_type_by_id(&self, id: &String) -> Option<ProductType> {
        let conn = &mut establish_connection();

        let result = product_type::table.filter(product_type::id.eq(id))
            .select(ProductType::as_select())
            .first::<ProductType>(conn)
            .optional();

        match result {
            Ok(Some(product_type)) => Some(product_type),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find product type with error: {}", e);
                None
            }
        }
    }

    pub fn create(&self, created_by: &String, name: &String, active: &bool) -> Option<ProductType> {
        let conn = &mut establish_connection();
        diesel::insert_into(crate::schema::product_type::table)
            .values( ProductType {
                id: self.id.clone(),
                name: name.clone(),
                active: Some(*active),
                created_by: created_by.clone(),
                updated_by: created_by.clone(),
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
            .execute(conn)
            .expect("Error saving new product type");

        self.get_product_type_by_id(&self.id)
    }
}