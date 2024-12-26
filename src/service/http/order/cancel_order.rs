use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, ErrorResponse};
use crate::repository::sale_order::{convert_sale_order_status_to_int, SaleOrder, SaleOrderStatus};

#[derive(Deserialize, Validate)]
pub struct CancelOrderRequest {
    sale_order_name: String
}

#[derive(Serialize)]
pub struct CancelOrderResponse {
    id: i32
}

pub async fn cancel_order(payload: web::Json<CancelOrderRequest>) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    match SaleOrder::new().get_by_sale_order_name(&payload.sale_order_name) {
        Some(sale_order) => {
            if sale_order.status.unwrap_or(0) != convert_sale_order_status_to_int(&SaleOrderStatus::SaleOrderStatusCreated) {
                return  HttpResponse::BadRequest().json(ErrorResponse {
                    is_error: true,
                    message: "sale order is not created state".to_string(),
                })
            }

            sale_order.update_status(&sale_order.id, &SaleOrderStatus::SaleOrderStatusCanceled);

            HttpResponse::Ok().json( CancelOrderResponse {
                id: sale_order.id,
            })
        }
        None => {
            HttpResponse::BadRequest().json(ErrorResponse {
                is_error: true,
                message: "sale order not found".to_string(),
            })
        }
    }
}