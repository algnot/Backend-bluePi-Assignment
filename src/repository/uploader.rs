use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::di::database::establish_connection;
use diesel::prelude::*;
use log::warn;
use crate::schema::uploader;

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

    pub fn get_upload_by_id(&self, id: &String) -> Option<Uploader> {
        let conn = &mut establish_connection();
        let result = uploader::table.filter(uploader::id.eq(id))
            .select(Uploader::as_select())
            .first::<Uploader>(conn)
            .optional();

        match result {
            Ok(Some(uploader)) => Some(uploader),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find uploader with error: {}", e);
                None
            }
        }
    }

    pub fn upload(&self, content: &String) -> Option<Uploader> {
        let conn = &mut establish_connection();
        diesel::insert_into(uploader::table)
            .values(Uploader {
                id: self.id.clone(),
                body: content.clone(),
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
            .execute(conn)
            .expect("Error saving new uploader");

        self.get_upload_by_id(&self.id)
    }
}
