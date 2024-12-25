use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::repository::product::Product;

#[derive(Serialize)]
pub struct ListProductResponse {
    products: Vec<Product>,
}

pub async fn list_product() -> impl Responder {
    let products = Product::new().get_all_products();

    HttpResponse::Ok().json(ListProductResponse {
        products
    })
}
