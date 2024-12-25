use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, AuthTokenHeader, ErrorResponse};
use crate::repository::product::{CreateProductEnt, Product};

#[derive(Validate, Deserialize)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1))]
    id: String,
    #[validate(length(min = 1))]
    name: String,
    description: Option<String>,
    image_id: Option<String>,
    #[validate(range(min = 0.01))]
    price: f64,
    #[validate(range(min = 0.0))]
    quantity: f64,
    type_id: Option<String>,
    recommend: bool,
    active: bool
}

#[derive(Serialize)]
pub struct UpdateProductResponse {
    id: String,
}

pub async fn update_product(payload: web::Json<UpdateProductRequest>, auth: AuthTokenHeader) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let user = auth.user.unwrap();
    let updated_product = Product::new().update_product(&user.id, &payload.id,&CreateProductEnt {
        name: payload.name.clone(),
        description: payload.description.clone(),
        image_id: payload.image_id.clone(),
        price: payload.price.clone(),
        quantity: payload.quantity.clone(),
        type_id: payload.type_id.clone(),
        recommend: payload.recommend.clone(),
        active: payload.active.clone(),
    });

    match updated_product {
        Some(product) => {
            HttpResponse::Ok().json({
                UpdateProductResponse {
                    id: product.id.clone(),
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
