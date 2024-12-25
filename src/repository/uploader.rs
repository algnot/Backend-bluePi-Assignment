use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::uploader)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Uploader {
    pub id: String,
    pub body: String,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl Uploader {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            body: "".to_string(),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }
}
