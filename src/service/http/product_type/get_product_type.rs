use actix_web::{web, HttpResponse, Responder};

use serde::Serialize;
use crate::common::request::{get_meta_data, ErrorResponse, MetaDataResponse};
use crate::repository::product_type::ProductType;

#[derive(Serialize)]
pub struct GetProductTypeResponse {
    id: String,
    name: String,
    active: bool,
    image_url: String,
    metadata: Option<MetaDataResponse>
}

pub async fn get_product_type(path: web::Path<(String,)>) -> impl Responder {
    let product_type_id = path.into_inner().0;
    let product_type = ProductType::new().get_product_type_by_id(&product_type_id);

    match product_type {
        Some(product_type) => {
            HttpResponse::Ok().json({
                GetProductTypeResponse {
                    id: product_type.id.to_string(),
                    name: product_type.name.to_string(),
                    active: product_type.active.unwrap_or(false),
                    image_url: product_type.image_id.unwrap_or_else(|| "".parse().unwrap()),
                    metadata: get_meta_data(product_type.created_at, product_type.updated_at, &product_type.created_by, &product_type.updated_by),
                }
            })
        }
        None => {
            HttpResponse::NotFound().json(ErrorResponse {
                is_error: true,
                message: format!("product with id {} not found", product_type_id),
            })
        }
    }
}
