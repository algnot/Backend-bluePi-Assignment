use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use crate::repository::system_parameter::SystemParameter;

#[derive(Serialize)]
pub struct GetCoinResponse {
    pub coin_1: i32,
    pub coin_5: i32,
    pub coin_10: i32,
    pub bank_20: i32,
    pub bank_50: i32,
    pub bank_100: i32,
    pub bank_500: i32,
    pub bank_1000: i32,
}

pub fn get_all_coin() -> GetCoinResponse {
    let coin_1 = SystemParameter::new().get_by_key_name(&"coin-1".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let coin_5 = SystemParameter::new().get_by_key_name(&"coin-5".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let coin_10 = SystemParameter::new().get_by_key_name(&"coin-10".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);

    let bank_20 = SystemParameter::new().get_by_key_name(&"bank-20".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_50 = SystemParameter::new().get_by_key_name(&"bank-50".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_100 = SystemParameter::new().get_by_key_name(&"bank-100".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_500 = SystemParameter::new().get_by_key_name(&"bank-500".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);
    let bank_1000 = SystemParameter::new().get_by_key_name(&"bank-1000".to_string()).unwrap_or(SystemParameter::new()).key_value.parse::<i32>().unwrap_or(0);

    GetCoinResponse {
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