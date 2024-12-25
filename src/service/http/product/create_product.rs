use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::{convert_validate_error_to_response, AuthTokenHeader, ErrorResponse};
use crate::repository::product::{CreateProductEnt, Product};

#[derive(Validate, Deserialize)]
pub struct CreateProductRequest {
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
pub struct CreateProductResponse {
    id: String,
}

pub async fn create_product(payload: web::Json<CreateProductRequest>, auth: AuthTokenHeader) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    let user = auth.user.unwrap();
    let product = Product::new().create(&user.id, CreateProductEnt {
        name: payload.name.clone(),
        description: payload.description.clone(),
        image_id: payload.image_id.clone(),
        price: payload.price,
        quantity: payload.quantity,
        type_id: payload.type_id.clone(),
        recommend: payload.recommend,
        active: payload.active,
    });

    match product {
        Some(product) => HttpResponse::Ok().json(CreateProductResponse{id: product.id}),
        None => HttpResponse::BadRequest().json({
            ErrorResponse {
                is_error: true,
                message: "cannot create product try again".to_string(),
            }
        })
    }
}