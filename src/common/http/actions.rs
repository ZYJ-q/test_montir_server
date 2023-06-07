use std::collections::HashMap;

use serde_json::Value;
use chrono::Utc;
use mysql::*;

use crate::actors::adapters::binance::parase::{get_account_positions, get_income_data, get_open_orders, get_history_accounts};

use super::{db_data, get_account_sub, http_data, BinanceFuturesApi, HttpVenueApi};

#[warn(dead_code, unused_variables, unused_mut)]
pub async fn get_account(traders: HashMap<String, db_data::Trader>) -> http_data::AccountRe {
    // http池子、
    let mut name_api: HashMap<String, Box<dyn HttpVenueApi>> = HashMap::new();

    for (key, value) in &traders {
        match value.tra_venue.as_str() {
            "Binance" => match value.r#type.as_str() {
                "Futures" => {
                    name_api.insert(
                        String::from(key),
                        Box::new(BinanceFuturesApi::new(
                            "https://fapi.binance.com",
                            &value.api_key,
                            &value.secret_key,
                        )),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 预备数据
    let mut data: http_data::AccountRe = http_data::AccountRe::new();

    // 合成account数据
    let mut subs: Vec<http_data::Sub> = Vec::new();
    let mut equities = 0.0;
    let mut equities_eth = 0.0;
    let mut origins = 0.0;
    let mut day_pnls = 0.0;
    let mut week_pnls = 0.0;
    let now = Utc::now();
    let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));
    for (key, value) in &name_api {
        let name = key;
        let origin = &traders.get(name).unwrap().ori_balance;
        let id = &traders.get(name).unwrap().tra_id;
        let res = get_account_sub(value, name, id, origin.parse().unwrap()).await;
        match res {
            Some(sub) => {
                equities += sub.total_equity.parse::<f64>().unwrap();
                equities_eth += sub.total_equity_eth.parse::<f64>().unwrap();
                origins += origin.parse::<f64>().unwrap();
                // day_pnls += sub.day_pnl.parse::<f64>().unwrap();
                // week_pnls += sub.week_pnl.parse::<f64>().unwrap();
                subs.push(sub);
            }
            None => {
                continue;
            }
        }
    }
    data.subs = subs;
    data.total.time = date;
    data.total.equity_eth = equities_eth.to_string();
    data.total.net_worth = (equities / origins).to_string();
    data.total.net_worth_eth = (equities_eth / origins).to_string();
    data.total.equity = equities.to_string();
    // data.total.day_pnl = day_pnls.to_string();
    // data.total.week_pnl = week_pnls.to_string();
    // 发送信息
    return data;
}



// 获取账户持仓数据
#[warn(dead_code, unused_variables, unused_mut)]
pub async fn get_history_position(traders: HashMap<String, db_data::Trader>) -> Vec<Value> {
    // http池子、
    let mut name_api: HashMap<String, Box<dyn HttpVenueApi>> = HashMap::new();

    for (key, value) in &traders {
        match value.tra_venue.as_str() {
            "Binance" => match value.r#type.as_str() {
                "Futures" => {
                    name_api.insert(
                        String::from(key),
                        Box::new(BinanceFuturesApi::new(
                            "https://fapi.binance.com",
                            &value.api_key,
                            &value.secret_key,
                        )),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 预备数据
    let mut data: Vec<Value> = [].to_vec() ;

    // 合成account数据
    for (key, value) in &name_api {
        let name = key;
        let origin = &traders.get(name).unwrap().ori_balance;
        let id = &traders.get(name).unwrap().tra_id;
        let res = get_account_positions(value, name, id, origin.parse().unwrap()).await;
        data = res
    }
    // 发送信息
    ;
    return data;
}


// 获取账户当前挂单数据
#[warn(dead_code, unused_variables, unused_mut)]
pub async fn get_history_open_order(traders: HashMap<String, db_data::Trader>) -> Vec<Value> {
    // http池子、
    let mut name_api: HashMap<String, Box<dyn HttpVenueApi>> = HashMap::new();

    for (key, value) in &traders {
        match value.tra_venue.as_str() {
            "Binance" => match value.r#type.as_str() {
                "Futures" => {
                    name_api.insert(
                        String::from(key),
                        Box::new(BinanceFuturesApi::new(
                            "https://fapi.binance.com",
                            &value.api_key,
                            &value.secret_key,
                        )),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 预备数据
    let mut data: Vec<Value> = [].to_vec() ;

    // 合成account数据
    for (key, value) in &name_api {
        let name = key;
        let origin = &traders.get(name).unwrap().ori_balance;
        let id = &traders.get(name).unwrap().tra_id;
        let res = get_open_orders(value, name, id, origin.parse().unwrap()).await;
        data = res
    }
    // 发送信息
    ;
    return data;
}



// 获取账户资产数据
#[warn(dead_code, unused_variables, unused_mut)]
pub async fn get_history_account(traders: HashMap<String, db_data::Trader>) -> Vec<Value> {
    // http池子、
    let mut name_api: HashMap<String, Box<dyn HttpVenueApi>> = HashMap::new();

    for (key, value) in &traders {
        match value.tra_venue.as_str() {
            "Binance" => match value.r#type.as_str() {
                "Futures" => {
                    name_api.insert(
                        String::from(key),
                        Box::new(BinanceFuturesApi::new(
                            "https://fapi.binance.com",
                            &value.api_key,
                            &value.secret_key,
                        )),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 预备数据
    let mut data: Vec<Value> = [].to_vec() ;

    // 合成account数据
    for (key, value) in &name_api {
        let name = key;
        let origin = &traders.get(name).unwrap().ori_balance;
        let id = &traders.get(name).unwrap().tra_id;
        let res = get_history_accounts(value, name, id, origin.parse().unwrap()).await;
        data = res
    }
    // 发送信息
    ;
    return data;
}

#[warn(dead_code, unused_variables, unused_mut)]
// 获取账户转账历史记录
pub async fn get_history_income(traders: HashMap<String, db_data::Trader>) -> Vec<Value> {
    // http池子、
    let mut name_api: HashMap<String, Box<dyn HttpVenueApi>> = HashMap::new();

    println!("数据{:?}", traders);



    for (key, value) in &traders {
        println!("value.tra_venue.as_str{}", value.tra_venue.as_str());
        println!("value.r#type.as_str{}", value.r#type.as_str());
        println!("Key{}", key);
        println!("api_Key{}", &value.api_key);
        println!("&value.secret_key{}", &value.secret_key);
        match value.tra_venue.as_str() {
            "Binance" => match value.r#type.as_str() {
                "Futures" => {
                    name_api.insert(
                        String::from(key),
                        Box::new(BinanceFuturesApi::new(
                            "https://api.binance.com",
                            &value.api_key,
                            &value.secret_key,
                        )),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    // 预备数据
    let mut data: Vec<Value> = [].to_vec() ;

    // 合成account数据
    for (key, value) in &name_api {
        let name = key;
        let id = &traders.get(name).unwrap().tra_id;
        let res = get_income_data(value, name, id).await;
        data = res
    }
    // 发送信息
    ;
    return data;
}

