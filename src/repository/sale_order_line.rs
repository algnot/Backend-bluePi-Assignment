use chrono::{NaiveDateTime, Utc};
use diesel::{BoolExpressionMethods, Insertable, Queryable, RunQueryDsl, Selectable};
use log::warn;
use serde::Serialize;
use crate::di::database::establish_connection;
use crate::repository::product::Product;
use crate::schema::sale_order_line;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order_line)]
pub struct SaleOrderLine {
    pub id: i32,
    pub product_id: String,
    pub sale_order_id: i32,
    pub quantity: Option<i32>,
    pub total: Option<f64>,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order_line)]
pub struct NewSaleOrderLine {
    pub product_id: String,
    pub sale_order_id: i32,
    pub quantity: i32,
    pub total: f64,
}

impl SaleOrderLine {
    pub fn new() -> Self {
        Self {
            id: 0,
            product_id: "".to_string(),
            sale_order_id: 0,
            quantity: Option::from(0),
            total: Option::from(0.0),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }

    pub fn get_by_sale_order_id_and_product_id(&self, sale_order_id: &i32, product_id: &String) -> Option<SaleOrderLine>  {
        let conn = &mut establish_connection();

        let result = sale_order_line::table
            .filter(sale_order_line::sale_order_id.eq(sale_order_id).and(sale_order_line::product_id.eq(product_id)))
            .select(SaleOrderLine::as_select())
            .first(conn)
            .optional();

        match result {
            Ok(Some(sale_order_line)) => Some(sale_order_line),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find sale order line with error: {}", e);
                None
            }
        }
    }

    pub fn get_by_sale_order_id(&self, sale_order_id: &i32) -> Vec<SaleOrderLine>  {
        let conn = &mut establish_connection();

        let result = sale_order_line::table
            .filter(sale_order_line::sale_order_id.eq(sale_order_id))
            .select(SaleOrderLine::as_select())
            .load(conn);

        result.unwrap_or_else(|e| {
            warn!("Cannot find sale order line with error: {}", e);
            Vec::new()
        })
    }

    pub fn create(&self, product_id: &String, sale_order_id: &i32, quantity: &i32) -> Option<SaleOrderLine> {
        match Product::new().get_product_by_id(product_id) {
            Some(product) => {
                let conn = &mut establish_connection();

                if product.quantity.unwrap_or(0.0) < *quantity as f64 {
                    return None;
                }

                diesel::insert_into(sale_order_line::table)
                    .values(NewSaleOrderLine {
                        product_id: product.id.clone(),
                        sale_order_id: sale_order_id.clone(),
                        quantity: quantity.clone(),
                        total: product.price? * quantity.clone() as f64,
                    })
                    .execute(conn)
                    .expect("Error saving new sale order line");

                Product::new().update_product_quantity(&product.id, &(product.quantity.unwrap_or(0.0) - *quantity as f64));
                self.get_by_sale_order_id_and_product_id(sale_order_id, product_id)
            }
            None => {
                warn!("{}", format!("product does not exist: {}", product_id));
                None
            }
        }
    }
}
