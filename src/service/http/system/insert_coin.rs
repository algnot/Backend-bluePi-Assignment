use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::repository::system_parameter::SystemParameter;
use crate::service::http::system::get_coin::{get_all_coin, GetCoinResponse};

#[derive(Deserialize, Validate)]
pub struct InsertCoinRequest {
    #[validate(range(min = 0))]
    coin_1: i32,
    #[validate(range(min = 0))]
    coin_5: i32,
    #[validate(range(min = 0))]
    coin_10: i32,
    #[validate(range(min = 0))]
    bank_20: i32,
    #[validate(range(min = 0))]
    bank_50: i32,
    #[validate(range(min = 0))]
    bank_100: i32,
    #[validate(range(min = 0))]
    bank_500: i32,
    #[validate(range(min = 0))]
    bank_1000: i32,
}

#[derive(Serialize)]
pub struct InsertCoinResponse {
    info: GetCoinResponse,
    total: i32
}

pub async fn insert_coin(payload: web::Json<InsertCoinRequest>) -> impl Responder {
    let all_coin = get_all_coin();

    SystemParameter::new().update_by_key_value(&"coin-1".to_string(), &(all_coin.coin_1 + payload.coin_1).to_string());
    SystemParameter::new().update_by_key_value(&"coin-5".to_string(), &(all_coin.coin_5 + payload.coin_5).to_string());
    SystemParameter::new().update_by_key_value(&"coin-10".to_string(), &(all_coin.coin_10 + payload.coin_10).to_string());
    SystemParameter::new().update_by_key_value(&"bank-20".to_string(), &(all_coin.bank_20 + payload.bank_20).to_string());
    SystemParameter::new().update_by_key_value(&"bank-50".to_string(), &(all_coin.bank_50 + payload.bank_50).to_string());
    SystemParameter::new().update_by_key_value(&"bank-100".to_string(), &(all_coin.bank_100 + payload.bank_100).to_string());
    SystemParameter::new().update_by_key_value(&"bank-500".to_string(), &(all_coin.bank_500 + payload.bank_500).to_string());
    SystemParameter::new().update_by_key_value(&"bank-1000".to_string(), &(all_coin.bank_1000 + payload.bank_1000).to_string());

    let total =
            (all_coin.coin_1 + payload.coin_1) * 1 +
            (all_coin.coin_5 + payload.coin_5) * 5 +
            (all_coin.coin_10 + payload.coin_10) * 10 +
            (all_coin.bank_20 + payload.bank_20) * 20 +
            (all_coin.bank_50 + payload.bank_50) * 50 +
            (all_coin.bank_100 + payload.bank_100) * 100 +
            (all_coin.bank_500 + payload.bank_500) * 500 +
            (all_coin.bank_1000 + payload.bank_1000) * 1000;

    HttpResponse::Ok().json(InsertCoinResponse {
        info: get_all_coin(),
        total,
    })
}