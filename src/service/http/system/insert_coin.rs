use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::convert_validate_error_to_response;
use crate::repository::system_parameter::SystemParameter;
use crate::service::http::system::get_coin::{get_all_coin, Coin};

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
    pub(crate) info: Coin,
    total: i32
}

pub fn insert_coin_to_db(coin: &Coin) -> InsertCoinResponse {
    let all_coin = get_all_coin();

    SystemParameter::new().update_by_key_value(&"coin-1".to_string(), &(all_coin.coin_1 + coin.coin_1).to_string());
    SystemParameter::new().update_by_key_value(&"coin-5".to_string(), &(all_coin.coin_5 + coin.coin_5).to_string());
    SystemParameter::new().update_by_key_value(&"coin-10".to_string(), &(all_coin.coin_10 + coin.coin_10).to_string());
    SystemParameter::new().update_by_key_value(&"bank-20".to_string(), &(all_coin.bank_20 + coin.bank_20).to_string());
    SystemParameter::new().update_by_key_value(&"bank-50".to_string(), &(all_coin.bank_50 + coin.bank_50).to_string());
    SystemParameter::new().update_by_key_value(&"bank-100".to_string(), &(all_coin.bank_100 + coin.bank_100).to_string());
    SystemParameter::new().update_by_key_value(&"bank-500".to_string(), &(all_coin.bank_500 + coin.bank_500).to_string());
    SystemParameter::new().update_by_key_value(&"bank-1000".to_string(), &(all_coin.bank_1000 + coin.bank_1000).to_string());

    let total =
        (all_coin.coin_1 + coin.coin_1) * 1 +
        (all_coin.coin_5 + coin.coin_5) * 5 +
        (all_coin.coin_10 + coin.coin_10) * 10 +
        (all_coin.bank_20 + coin.bank_20) * 20 +
        (all_coin.bank_50 + coin.bank_50) * 50 +
        (all_coin.bank_100 + coin.bank_100) * 100 +
        (all_coin.bank_500 + coin.bank_500) * 500 +
        (all_coin.bank_1000 + coin.bank_1000) * 1000;

    InsertCoinResponse {
        info: get_all_coin(),
        total,
    }
}

pub async fn insert_coin(payload: web::Json<Coin>) -> impl Responder {
    if let Err(errors) = payload.validate() {
        return HttpResponse::BadRequest().json(convert_validate_error_to_response(errors));
    }

    HttpResponse::Ok().json(insert_coin_to_db(&payload))
}