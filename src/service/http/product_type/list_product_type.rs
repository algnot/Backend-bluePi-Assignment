use actix_web::{HttpResponse, Responder};
use serde::{Serialize};
use crate::repository::product_type::ProductType;

#[derive(Serialize)]
pub struct ListProductTypeResponse {
    product_type_list: Vec<ProductType>,
}

pub async fn list_product_type() -> impl Responder {
    let product_type = ProductType::new().get_all_product_type();

    HttpResponse::Ok().json({
        ListProductTypeResponse {
            product_type_list: product_type
        }
    })
}
