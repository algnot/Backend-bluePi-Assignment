use crate::repository::product::Product;
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use serde::{Serialize};
use crate::di::database::establish_connection;
use log::warn;
use rand::Rng;
use diesel::prelude::*;
use crate::repository::sale_order_line::SaleOrderLine;
use crate::schema::{sale_order};
use crate::schema::sale_order::sale_order_name;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order)]
pub struct SaleOrder {
    pub id: i32,
    pub sale_order_name: String,
    pub status: Option<i32>,
    pub total:  Option<f64>,

    // meta data
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::sale_order)]
pub struct NewSaleOrder {
    pub sale_order_name: String,
    pub total: f64,
}

#[derive(PartialEq)]
pub enum SaleOrderStatus {
    SaleOrderStatusCreated,
    SaleOrderStatusPaid,
    SaleOrderStatusCanceled,
}

pub fn convert_sale_order_status_to_int(status: &SaleOrderStatus) -> i32 {
    match status {
        SaleOrderStatus::SaleOrderStatusCreated => 0,
        SaleOrderStatus::SaleOrderStatusPaid => 1,
        SaleOrderStatus::SaleOrderStatusCanceled => 2
    }
}

pub fn convert_sale_order_status_to_string(status: i32) -> String {
    if status == SaleOrderStatus::SaleOrderStatusCreated as i32 {
        String::from("Created")
    }
    else if status == SaleOrderStatus::SaleOrderStatusPaid as i32 {
        String::from("Paid")
    }
    else if status == SaleOrderStatus::SaleOrderStatusCanceled as i32 {
        String::from("Canceled")
    }
    else {
        String::from("Unknown")
    }
}

#[derive(Serialize)]
pub struct OrderLineResponse {
    product_id: String,
    image_id: String,
    name: String,
    quantity: i32,
    unit_price: f64,
    total: f64,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct OrderResponse {
    id: i32,
    sale_order_name: String,
    status: String,
    total: f64,
    order_line: Vec<OrderLineResponse>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl SaleOrder {
    pub fn new() -> Self {
        Self {
            id: 0,
            sale_order_name: Self::generate_sale_order_name(),
            status: Option::from(0),
            total: Option::from(0.0),
            created_at: Option::from(Utc::now().naive_utc()),
            updated_at: Option::from(Utc::now().naive_utc()),
        }
    }

    pub fn generate_sale_order_name() -> String {
        let date_part = Utc::now().format("%Y%m%d").to_string();
        let random_part: u32 = rand::thread_rng().gen_range(10000..100000);
        format!("SO{}{}", date_part, random_part)
    }

    pub fn get_by_sale_order_name(&self, name: &String) -> Option<SaleOrder> {
        let conn = &mut establish_connection();

        let result = sale_order::table
            .filter(sale_order_name.eq(name))
            .select(SaleOrder::as_select())
            .first(conn)
            .optional();

        match result {
            Ok(Some(sale_order)) => Some(sale_order),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find sale order with error: {}", e);
                None
            }
        }
    }

    pub fn get_by_id(&self, id: &i32) -> Option<SaleOrder> {
        let conn = &mut establish_connection();

        let result = sale_order::table
            .filter(sale_order::id.eq(id))
            .select(SaleOrder::as_select())
            .first(conn)
            .optional();

        match result {
            Ok(Some(sale_order)) => Some(sale_order),
            Ok(None) => None,
            Err(e) => {
                warn!("Cannot find sale order with error: {}", e);
                None
            }
        }
    }

    pub fn to_response(&self) -> OrderResponse {
        let sale_order_lines = SaleOrderLine::new().get_by_sale_order_id(&self.id);
        let mut order_line: Vec<OrderLineResponse> = vec![];

        for sale_order_line in sale_order_lines {
            let product = Product::new().get_product_by_id(&sale_order_line.product_id).unwrap_or(Product::new());
            order_line.push(OrderLineResponse {
                product_id: sale_order_line.product_id,
                image_id: product.image_id.unwrap_or("".to_string()),
                name: product.name,
                quantity: sale_order_line.quantity.unwrap_or(0),
                unit_price: sale_order_line.total.unwrap_or(0.0) / sale_order_line.quantity.unwrap_or(1) as f64,
                total: sale_order_line.total.unwrap_or(0.0),
                created_at: sale_order_line.created_at,
                updated_at: sale_order_line.updated_at,
            })
        }

        OrderResponse {
            id: self.id.clone(),
            sale_order_name: self.sale_order_name.clone(),
            status: convert_sale_order_status_to_string(self.status.unwrap_or(99)),
            total: self.total.unwrap_or(0.0),
            order_line,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn create(&self) -> Option<SaleOrder> {
        let conn = &mut establish_connection();
        diesel::insert_into(sale_order::table)
            .values( NewSaleOrder {
                sale_order_name: self.sale_order_name.clone(),
                total: 0.0,
            })
            .execute(conn)
            .expect("Error saving new system parameter");

        self.get_by_sale_order_name(&self.sale_order_name)
    }

    pub fn update_status(&self, sale_order_id: &i32, status: &SaleOrderStatus) -> Option<SaleOrder> {
        let conn = &mut establish_connection();

        let update_result = diesel::update(sale_order::table.filter(sale_order::id.eq(sale_order_id)))
            .set((
                sale_order::status.eq(convert_sale_order_status_to_int(status)),
            ))
            .execute(conn);

        if status == &SaleOrderStatus::SaleOrderStatusCanceled {
            let sale_order_line = SaleOrderLine::new().get_by_sale_order_id(sale_order_id);

            for sale_order_line in sale_order_line {
                let product = Product::new().get_product_by_id(&sale_order_line.product_id).unwrap_or(Product::new());
                product.update_product_quantity(&product.id, &(sale_order_line.quantity.unwrap_or(0) as f64 + product.quantity.unwrap_or(0.0)));
            }
        }

        match update_result {
            Ok(rows_updated) if rows_updated > 0 => {
                self.get_by_id(sale_order_id)
            }
            Ok(_) => {
                warn!("No sale order found with ID: {}", sale_order_id);
                None
            }
            Err(e) => {
                warn!("Failed to update sale order with ID {}: {}", sale_order_id, e);
                None
            }
        }
    }

    pub fn update_total(&self, sale_order_id: &i32, total: &f64) -> Option<SaleOrder> {
        let conn = &mut establish_connection();

        let update_result = diesel::update(sale_order::table.filter(sale_order::id.eq(sale_order_id)))
            .set((
                sale_order::total.eq(total),
            ))
            .execute(conn);

        match update_result {
            Ok(rows_updated) if rows_updated > 0 => {
                self.get_by_id(sale_order_id)
            }
            Ok(_) => {
                warn!("No sale order found with ID: {}", sale_order_id);
                None
            }
            Err(e) => {
                warn!("Failed to update sale order with ID {}: {}", sale_order_id, e);
                None
            }
        }
    }
}
