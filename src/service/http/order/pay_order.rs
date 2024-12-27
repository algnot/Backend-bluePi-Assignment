use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::request::ErrorResponse;
use crate::repository::sale_order::{convert_sale_order_status_to_int, SaleOrder, SaleOrderStatus};
use crate::service::http::system::get_coin::{to_total, Coin};
use crate::service::http::system::insert_coin::insert_coin_to_db;

#[derive(Deserialize, Validate)]
pub struct PayOrderRequest {
    sale_order_name: String,
    user_coin: Coin
}

#[derive(Serialize)]
pub struct PayOrderResponse {
    is_error: bool,
    message: String,
    change_coin: Coin
}

pub fn clone_coin(coin: &Coin) -> Coin {
    Coin {
        coin_1: coin.coin_1.clone(),
        coin_5: coin.coin_5.clone(),
        coin_10: coin.coin_10.clone(),
        bank_20: coin.bank_20.clone(),
        bank_50: coin.bank_50.clone(),
        bank_100: coin.bank_100.clone(),
        bank_500: coin.bank_500.clone(),
        bank_1000: coin.bank_1000.clone(),
    }
}

pub fn calculate_change_coin(all_coin: &mut Coin, change_amount: &f64) -> Coin {
    let mut remaining_amount = *change_amount;
    let mut result = Coin {
        coin_1: 0,
        coin_5: 0,
        coin_10: 0,
        bank_20: 0,
        bank_50: 0,
        bank_100: 0,
        bank_500: 0,
        bank_1000: 0,
    };
    
    let mut denominations = [
        (1000, &mut all_coin.bank_1000, &mut result.bank_1000),
        (500, &mut all_coin.bank_500, &mut result.bank_500),
        (100, &mut all_coin.bank_100, &mut result.bank_100),
        (50, &mut all_coin.bank_50, &mut result.bank_50),
        (20, &mut all_coin.bank_20, &mut result.bank_20),
        (10, &mut all_coin.coin_10, &mut result.coin_10),
        (5, &mut all_coin.coin_5, &mut result.coin_5),
        (1, &mut all_coin.coin_1, &mut result.coin_1),
    ];

    for (value, available, to_give) in denominations.iter_mut() {
        if remaining_amount >= *value as f64 {
            let needed = (remaining_amount / *value as f64).floor() as i32;
            if **available >= needed {
                **to_give += needed;
                **available -= needed;
                remaining_amount -= needed as f64 * *value as f64;
            }
        }
    }

    result
}

pub async fn pay_order(payload: web::Json<PayOrderRequest>) -> impl Responder {
    match SaleOrder::new().get_by_sale_order_name(&payload.sale_order_name) {
        Some(sale_order) => {
            let sale_order_amount = sale_order.total.unwrap_or(0.0);
            let user_amount = to_total(&payload.user_coin);

            if sale_order.status.unwrap_or(2) == convert_sale_order_status_to_int(&SaleOrderStatus::SaleOrderStatusCanceled) {
                return HttpResponse::Ok().json({
                    PayOrderResponse {
                        is_error: true,
                        message: "sale order is canceled".to_string(),
                        change_coin: clone_coin(&payload.user_coin)
                    }
                })
            }

            if user_amount < sale_order_amount {
                sale_order.update_status(&sale_order.id, &SaleOrderStatus::SaleOrderStatusCanceled);
                return HttpResponse::Ok().json({
                    PayOrderResponse {
                        is_error: true,
                        message: "sale order amount is more then user amount".to_string(),
                        change_coin: clone_coin(&payload.user_coin)
                    }
                })
            }

            let change_amount = user_amount - sale_order_amount;
            let mut all_coin= insert_coin_to_db(&payload.user_coin).info;

            let change_coin = calculate_change_coin(&mut all_coin, &change_amount);
            if to_total(&change_coin) != change_amount {
                sale_order.update_status(&sale_order.id, &SaleOrderStatus::SaleOrderStatusCanceled);
                insert_coin_to_db(&Coin {
                    coin_1: &payload.user_coin.coin_1 * -1,
                    coin_5: &payload.user_coin.coin_5 * -1,
                    coin_10: &payload.user_coin.coin_10 * -1,
                    bank_20: &payload.user_coin.bank_20 * -1,
                    bank_50: &payload.user_coin.bank_50 * -1,
                    bank_100: &payload.user_coin.bank_100 * -1,
                    bank_500: &payload.user_coin.bank_500 * -1,
                    bank_1000: &payload.user_coin.bank_1000 * -1,
                });
                return HttpResponse::Ok().json({
                    PayOrderResponse {
                        is_error: true,
                        message: "there are not enough coins for change system will return your coin".to_string(),
                        change_coin: clone_coin(&payload.user_coin)
                    }
                })
            }

            insert_coin_to_db(&Coin {
                coin_1: change_coin.coin_1 * -1,
                coin_5: change_coin.coin_5 * -1,
                coin_10: change_coin.coin_10 * -1,
                bank_20: change_coin.bank_20 * -1,
                bank_50: change_coin.bank_50 * -1,
                bank_100: change_coin.bank_100 * -1,
                bank_500: change_coin.bank_500 * -1,
                bank_1000: change_coin.bank_1000 * -1,
            });
            sale_order.update_status(&sale_order.id, &SaleOrderStatus::SaleOrderStatusPaid);

            HttpResponse::Ok().json({
                PayOrderResponse {
                    is_error: false,
                    message: "payment completed".to_string(),
                    change_coin,
                }
            })
        }
        None => {
            HttpResponse::BadRequest().json({
                ErrorResponse {
                    is_error: false,
                    message: format!("SaleOrder {} not found", payload.sale_order_name),
                }
            })
        }
    }
}