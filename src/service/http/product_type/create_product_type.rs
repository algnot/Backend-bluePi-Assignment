use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, AuthTokenHeader, ErrorResponse};
use crate::repository::product_type::ProductType;

#[derive(Validate, Deserialize)]
pub struct CreateProductTypeRequest {
    #[validate(length(min = 1))]
    name: String,
    image_id: String,
    active: bool,
}

#[derive(Serialize)]
pub struct CreateProductTypeResponse {
    id: String,
}

pub async fn create_product_type(payload: web::Json<CreateProductTypeRequest>, auth: AuthTokenHeader) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let user = auth.user.unwrap();
    let product_type = ProductType::new().create(&user.id, &payload.name, &payload.image_id, &payload.active);

    match product_type {
        Some(product_type) => {
            HttpResponse::Ok().json( CreateProductTypeResponse {
                id: product_type.id.to_string(),
            })
        }
        None => {
            HttpResponse::BadRequest().json( ErrorResponse {
                is_error: true,
                message: "cannot create product type try again!".to_string(),
            })
        }
    }
}
