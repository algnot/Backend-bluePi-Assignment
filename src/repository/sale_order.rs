use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order)]
pub struct SaleOrder {
    pub id: i32,
    pub sale_order_name: String,
    pub status: i32,
    pub total: f64,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl SaleOrder {
    pub fn new() -> Self {
        Self {
            id: 0,
            sale_order_name: "".to_string(),
            status: 0,
            total: 0.0,
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }
}
