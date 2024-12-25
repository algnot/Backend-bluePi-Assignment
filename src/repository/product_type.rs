use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use log::warn;
use serde::Serialize;
use diesel::prelude::*;
use crate::di::database::establish_connection;
use crate::schema::product_type;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::product_type)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProductType {
    pub id: String,
    pub name: String,
    pub active: Option<bool>,
    pub image_id: Option<String>,

    // meta data
    pub created_by: String,
    pub updated_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

pub struct UpdateProductTypeEnt {
    pub(crate) name: String,
    pub(crate) active: bool,
    pub(crate) image_id: Option<String>,
}

impl ProductType {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            active: Some(true),
            image_id: None,
            created_by: "".to_string(),
            updated_by: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }

    pub fn update_product_type(&self, updated_by: &String, id: &String, value: &UpdateProductTypeEnt) -> Option<ProductType> {
        let conn = &mut establish_connection();

        let update_result = diesel::update(product_type::table.filter(product_type::id.eq(id)))
            .set((
                product_type::name.eq(&value.name),
                product_type::active.eq(&value.active),
                product_type::updated_by.eq(updated_by),
                product_type::image_id.eq(&value.image_id),
                product_type::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn);

        match update_result {
            Ok(rows_updated) if rows_updated > 0 => {
                self.get_product_type_by_id(id)
            }
            Ok(_) => {
                warn!("No product type found with ID: {}", id);
                None
            }
            Err(e) => {
                warn!("Failed to update product type with ID {}: {}", id, e);
                None
            }
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

    pub fn get_all_product_type(&self) -> Vec<ProductType> {
        let conn = &mut establish_connection();

        product_type::table
            .select(ProductType::as_select())
            .load::<ProductType>(conn).unwrap_or_else(|e| {
            warn!("Error retrieving product types: {}", e);
            vec![]
        })
    }

    pub fn create(&self, created_by: &String, name: &String, image_id: &String, active: &bool) -> Option<ProductType> {
        let conn = &mut establish_connection();
        diesel::insert_into(product_type::table)
            .values( ProductType {
                id: self.id.clone(),
                name: name.clone(),
                active: Some(*active),
                image_id: Option::from(image_id.clone()),
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