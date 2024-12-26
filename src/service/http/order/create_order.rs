use std::collections::HashSet;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, ErrorResponse};
use crate::repository::sale_order::{SaleOrder, SaleOrderStatus};
use crate::repository::sale_order_line::SaleOrderLine;

#[derive(Deserialize, Validate)]
pub struct OrderLineRequest {
    product_id: String,
    #[validate(range(min = 0))]
    quantity: i32,
}

#[derive(Deserialize, Validate)]
pub struct CreateOrderRequest {
    order_line: Vec<OrderLineRequest>,
}

pub async fn create_order(payload: web::Json<CreateOrderRequest>) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let mut seen_products = HashSet::new();
    for order_line in &payload.order_line {
        if !seen_products.insert(&order_line.product_id) {
            return HttpResponse::BadRequest().json(ErrorResponse {
                is_error: true,
                message: format!("product {} is duplicated", order_line.product_id),
            });
        }
    }

    match SaleOrder::new().create() {
        Some(sale_order) => {
            let mut total = 0.0;

            for order_line in payload.order_line.iter() {
                let sale_order_line = SaleOrderLine::new().create(&order_line.product_id, &sale_order.id, &order_line.quantity);

                match sale_order_line {
                    Some(sale_order_line) => {
                        total = total + sale_order_line.total.unwrap_or(0.0)
                    }
                    None => {
                        return HttpResponse::BadRequest().json({
                            sale_order.update_status(&sale_order.id, &SaleOrderStatus::SaleOrderStatusCanceled);
                            ErrorResponse {
                                is_error: true,
                                message: format!("cannot found product id {} or not enough quantity", order_line.product_id),
                            }
                        })
                    }
                }
            }

            let updated_sale_order = sale_order.update_total(&sale_order.id, &total);
            match updated_sale_order {
                Some(updated_sale_order) => {
                    HttpResponse::Ok().json(updated_sale_order.to_response())
                }
                None => {
                    HttpResponse::InternalServerError().json("Something went wrong")
                }
            }
        },
        None => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}