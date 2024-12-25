use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, AuthTokenHeader, ErrorResponse};
use crate::repository::product_type::{ProductType, UpdateProductEnt};

#[derive(Validate, Deserialize)]
pub struct UpdateProductTypeRequest {
    #[validate(length(min = 1))]
    id: String,
    #[validate(length(min = 1))]
    name: String,
    active: bool,
}

#[derive(Serialize)]
pub struct UpdateProductTypeResponse {
    id: String,
}

pub async fn update_product_type(payload: web::Json<UpdateProductTypeRequest>, auth: AuthTokenHeader) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let user = auth.user.unwrap();
    let updated_product_type = ProductType::new().update_product(&user.id, &payload.id, &UpdateProductEnt {
        name: payload.name.clone(),
        active: payload.active.clone(),
    });

    match updated_product_type {
        Some(updated_product_type) => {
            HttpResponse::Ok().json({
                UpdateProductTypeResponse {
                    id: updated_product_type.id.clone(),
                }
            })
        }
        None => {
            HttpResponse::BadRequest().json(ErrorResponse {
                is_error: true,
                message: format!("product with id {} not found", payload.id),
            })
        }
    }
}