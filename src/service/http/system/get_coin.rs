use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::repository::system_parameter::SystemParameter;

#[derive(Serialize, Deserialize, Validate)]
pub struct Coin {
    #[validate(range(min = 0))]
    pub coin_1: i32,
    #[validate(range(min = 0))]
    pub coin_5: i32,
    #[validate(range(min = 0))]
    pub coin_10: i32,
    #[validate(range(min = 0))]
    pub bank_20: i32,
    #[validate(range(min = 0))]
    pub bank_50: i32,
    #[validate(range(min = 0))]
    pub bank_100: i32,
    #[validate(range(min = 0))]
    pub bank_500: i32,
    #[validate(range(min = 0))]
    pub bank_1000: i32,
}

pub fn to_total(coin: &Coin) -> f64 {
    let mut  total = 0.0;
    total += coin.coin_1 as f64 * 1.0;
    total += coin.coin_5 as f64 * 5.0;
    total += coin.coin_10 as f64 * 10.0;
    total += coin.bank_20 as f64 * 20.0;
    total += coin.bank_50 as f64 * 50.0;
    total += coin.bank_100 as f64 * 100.0;
    total += coin.bank_500 as f64 * 500.0;
    total += coin.bank_1000 as f64 * 1000.0;
    total
}

pub fn get_all_coin() -> Coin {
    let coin_1 = SystemParameter::new().get_by_key_name(&"coin-1".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let coin_5 = SystemParameter::new().get_by_key_name(&"coin-5".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let coin_10 = SystemParameter::new().get_by_key_name(&"coin-10".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);

    let bank_20 = SystemParameter::new().get_by_key_name(&"bank-20".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_50 = SystemParameter::new().get_by_key_name(&"bank-50".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_100 = SystemParameter::new().get_by_key_name(&"bank-100".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_500 = SystemParameter::new().get_by_key_name(&"bank-500".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_1000 = SystemParameter::new().get_by_key_name(&"bank-1000".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);

    Coin {
        coin_1,
        coin_5,
        coin_10,
        bank_20,
        bank_50,
        bank_100,
        bank_500,
        bank_1000,
    }
}

pub async fn get_coin() -> impl Responder {
    HttpResponse::Ok().json({
        get_all_coin()
    })
}