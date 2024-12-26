use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order_line)]
pub struct SaleOrderLine {
    pub id: i32,
    pub product_id: String,
    pub sale_order_id: i32,
    pub quantity: i32,
    pub total: f64,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl SaleOrderLine {
    pub fn new() -> Self {
        Self {
            id: 0,
            product_id: "".to_string(),
            sale_order_id: 0,
            quantity: 0,
            total: 0.0,
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }
}
