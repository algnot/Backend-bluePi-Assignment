use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use diesel::prelude::*;
use log::warn;
use crate::di::database::establish_connection;
use crate::repository::product_type::ProductType;
use crate::schema::{product, product_type};

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::product)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<f64>,
    pub type_id: Option<String>,
    pub recommend: Option<bool>,
    pub active: Option<bool>,
    pub image_id: Option<String>,

    // meta data
    pub created_by: String,
    pub updated_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

pub struct CreateProductEnt {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_id: Option<String>,
    pub(crate) price: f64,
    pub(crate) quantity: f64,
    pub(crate) type_id: Option<String>,
    pub(crate) recommend: bool,
    pub(crate) active: bool,
}

impl Product {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            description: Some("".to_string()),
            price: Some(0.0),
            quantity: Some(0.0),
            type_id: Option::from("".to_string()),
            recommend: Some(false),
            active: Some(true),
            image_id: None,
            created_by: "".to_string(),
            updated_by: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }

    pub fn get_product_by_id(&self, id: &String) -> Option<Product> {
        let conn = &mut establish_connection();
        let result = product::table.filter(product::id.eq(id))
            .select(Product::as_select())
            .first::<Product>(conn)
            .optional();

        match result {
            Ok(Some(product)) => Some(product),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find product with error: {}", e);
                None
            }
        }
    }

    pub fn get_all_products(&self) -> Vec<Product> {
        let conn = &mut establish_connection();

        product::table
            .select(Product::as_select())
            .load::<Product>(conn).unwrap_or_else(|e| {
            warn!("Error retrieving products: {}", e);
            vec![]
        })
    }

    pub fn create(&self, created_by: &String, value: CreateProductEnt) -> Option<Product> {
        let conn = &mut establish_connection();
        diesel::insert_into(product::table)
            .values( Product {
                id: self.id.clone(),
                name: value.name,
                description: value.description,
                price: Option::from(value.price),
                quantity: Option::from(value.quantity),
                type_id: value.type_id,
                recommend: Option::from(value.recommend),
                active: Option::from(value.active),
                image_id: value.image_id,
                created_by: created_by.clone(),
                updated_by: created_by.clone(),
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
            .execute(conn)
            .expect("Error saving new product");

        self.get_product_by_id(&self.id)
    }
}