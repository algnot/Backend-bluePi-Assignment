use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

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

}