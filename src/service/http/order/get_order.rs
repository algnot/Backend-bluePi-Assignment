use actix_web::{web, HttpResponse, Responder};
use crate::common::request::ErrorResponse;
use crate::repository::sale_order::SaleOrder;

pub async fn get_order(path: web::Path<(String,)>) -> impl Responder {
    let sale_order_name = path.into_inner().0;
    let sale_order = SaleOrder::new().get_by_sale_order_name(&sale_order_name);

    match sale_order {
        Some(sale_order) => {
            HttpResponse::Ok().json(sale_order.to_response())
        }
        None => {
            HttpResponse::BadRequest().json( ErrorResponse {
                is_error: true,
                message: format!("Could not find order with sale order name: {}", sale_order_name),
            })
        }
    }
}