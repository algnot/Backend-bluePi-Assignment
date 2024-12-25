use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::uploader)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Uploader {
    pub id: String,
    pub url: String,
    pub ref_table: String,
    pub ref_id: String,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl Uploader {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url: "".to_string(),
            ref_table: "".to_string(),
            ref_id: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }
}
