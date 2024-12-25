use actix_web::{web, HttpResponse, Responder};
use diesel::serialize::IsNull::No;
use serde::Serialize;
use crate::common::request::{get_meta_data, ErrorResponse, MetaDataResponse};
use crate::repository::product::Product;
use crate::repository::product_type::ProductType;

#[derive(Serialize)]
pub struct GetProductTypeResponse {
    name: String,
    active: bool,
}

#[derive(Serialize)]
pub struct GetProductResponse {
    id: String,
    name: String,
    description: String,
    price: f64,
    quantity: f64,
    product_type: Option<GetProductTypeResponse>,
    recommend: Option<bool>,
    active: bool,
    image_id: Option<String>,
    metadata: Option<MetaDataResponse>
}

pub async fn get_product(path: web::Path<(String,)>) -> impl Responder {
    let product_id = path.into_inner().0;
    let product = Product::new().get_product_by_id(&product_id);

    match product {
        Some(product) => {
            let product_type = ProductType::new().get_product_type_by_id(&product.type_id.unwrap_or("".to_string())).unwrap_or(ProductType::new());

            HttpResponse::Ok().json({
                GetProductResponse {
                    id: product.id,
                    name: product.name,
                    description: product.description.unwrap_or("".parse().unwrap()),
                    price: product.price.unwrap_or(0.0),
                    quantity: product.quantity.unwrap_or(0.0),
                    product_type: Option::from(GetProductTypeResponse {
                        name: product_type.name.clone(),
                        active: product_type.active.clone().unwrap_or(false),
                    }),
                    recommend: product.recommend,
                    active: product.active.unwrap_or(false),
                    metadata: get_meta_data(product.created_at, product.updated_at, &product.created_by, &product.updated_by),
                    image_id: product.image_id,
                }
            })
        }
        None => {
            HttpResponse::NotFound().json({
                ErrorResponse {
                    is_error: true,
                    message: "product not found".to_string(),
                }
            })
        }
    }
}
