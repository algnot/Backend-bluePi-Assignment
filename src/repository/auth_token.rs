use chrono::{Duration, Utc};
use diesel::{Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use log::warn;
use crate::common::encryptor::encrypt;
use crate::common::jwt::{convert_token_type, create_jwt, TokenType};
use crate::di::database::establish_connection;
use crate::repository::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::schema::auth_token;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::auth_token)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AuthToken {
    pub id: String,
    pub uid: String,
    pub token_type: i32,
    pub iat: i32,
    pub exp: i32,
    pub is_revoke: Option<bool>,
}

impl AuthToken {
    pub fn new(token_type: &TokenType, uid: &String) -> Self {
        let current_time = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            uid: uid.clone(),
            token_type: convert_token_type(token_type),
            iat: current_time.timestamp() as i32,
            exp: current_time.timestamp() as i32,
            is_revoke: Some(false),
        }
    }

    pub fn save_to_database(&self) {
        let conn = &mut establish_connection();
        diesel::insert_into(auth_token::table)
            .values(AuthToken {
                id: self.id.clone(),
                uid: self.uid.clone(),
                token_type: self.token_type.clone(),
                iat: self.iat,
                exp: self.exp,
                is_revoke: self.is_revoke,
            })
            .execute(conn)
            .expect("Error saving token");
    }

    pub fn generate_token(&self) -> (String, String) {
        let now = Utc::now();

        // access token
        let mut access_token = AuthToken::new(&TokenType::AccessToken, &self.uid);
        let exp_access_token = now.checked_add_signed(Duration::minutes(15))
                                        .expect("valid timestamp")
                                        .timestamp();
        access_token.uid = self.uid.clone();
        access_token.iat = now.timestamp() as i32;
        access_token.exp = exp_access_token as i32;
        let access_token_sting = create_jwt(&access_token.uid,
                                                   &access_token.id,
                                                   TokenType::AccessToken,
                                                   access_token.iat as usize,
                                                   access_token.exp as usize).unwrap_or_else(|e| e.to_string());
        access_token.save_to_database();

        // refresh token
        let mut refresh_token = AuthToken::new(&TokenType::RefreshToken, &self.uid);
        let exp_refresh_token = now.checked_add_signed(Duration::days(7))
                                        .expect("valid timestamp")
                                        .timestamp();
        refresh_token.uid = self.uid.clone();
        refresh_token.iat = now.timestamp() as i32;
        refresh_token.exp = exp_refresh_token as i32;
        let refresh_token_sting = create_jwt(&refresh_token.uid,
                                                   &refresh_token.id,
                                                   TokenType::RefreshToken,
                                                   refresh_token.iat as usize,
                                                   refresh_token.exp as usize).unwrap_or_else(|e| e.to_string());
        refresh_token.save_to_database();

        (access_token_sting, refresh_token_sting)
    }

    pub fn get_token_info(&self, id: &String) -> Option<AuthToken> {
        let conn = &mut establish_connection();

        let result = auth_token::table
            .filter(auth_token::id.eq(id))
            .first::<AuthToken>(conn)
            .optional();

        match result {
            Ok(Some(auth_token)) => Some(auth_token),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find auth with error: {}", e);
                None
            }
        }
    }
}
