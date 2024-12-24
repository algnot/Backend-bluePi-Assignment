use chrono::Utc;
use diesel::prelude::*;
use log::{info, warn};
use crate::common::encryptor::{encrypt, hash_password};
use crate::di::database::establish_connection;
use crate::schema::users::dsl::users;
use crate::schema::users::{email};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: String,
    pub active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub username: Vec<u8>,
    pub email: Vec<u8>,
    pub hashed_password: Vec<u8>,
}

impl User {
    pub fn new() -> Self {
        let current_time = Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(current_time),
            updated_at: Some(current_time),
            username: vec![],
            email: vec![],
            active: Some(true),
            hashed_password: vec![],
        }
    }

    pub fn find_by_email(&self, value: &String) -> Option<User> {
        let conn = &mut establish_connection();

        let result = QueryDsl::filter(users, email.eq(encrypt(value)))
            .select(User::as_select())
            .first(conn)
            .optional();

        match result {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find user with error: {}", e);
                None
            }
        }
    }

    pub fn create(&self, email_value: &String, username_value: &String, password_value: &String) -> (Option<User>, bool) {
        if let Some(user) = self.find_by_email(email_value) {
            info!("Found existing user {}", email_value);
            return (Some(user), true)
        }

        let conn = &mut establish_connection();
        diesel::insert_into(crate::schema::users::table)
            .values(User {
                id: self.id.clone(),
                active: self.active.clone(),
                created_at: self.created_at,
                updated_at: self.updated_at,
                username: encrypt(username_value),
                email: encrypt(email_value),
                hashed_password: encrypt(&hash_password(password_value).expect("cannot hash password")),
            })
            .execute(conn)
            .expect("Error saving new user");

        (self.find_by_email(email_value), false)
    }
}
