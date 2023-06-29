use chrono::Local;
use log::error;
use serde_json::{Map,Value};
use std::collections::VecDeque;
use std::fs;
use chrono::{DateTime, NaiveDateTime, Utc};
use super::http_data::Sub;
use super::base::venue_api::HttpVenueApi;

pub async fn get_account_sub(
    http_api: &Box<dyn HttpVenueApi>,
    name: &str,
    id: &u64,
    origin_balance: f64,
) -> Option<Sub> {
    if let Some(data) = http_api.account().await {
        let value: Value = serde_json::from_str(&data).unwrap();
        // println!("账户信息{}", value);
        let assets = value.as_object().unwrap().get("assets")
        .unwrap().as_array().unwrap();
        let mut new_total_balance = 0.00;
        let mut new_total_equity = 0.00;
        let mut best_price = 0.00;
        for a in assets {
            let obj = a.as_object().unwrap();
            let wallet_balance: f64 = obj.get("walletBalance").unwrap().as_str().unwrap().parse().unwrap();
            let symbol = obj.get("asset").unwrap().as_str().unwrap();

            if wallet_balance != 0.00 {
                if symbol == "ETH" {
                    let asset = format!("{}USDT", symbol);
                    if let Some(data) = http_api.get_klines(&asset).await {
                        let v: Value = serde_json::from_str(&data).unwrap();
                        let price_obj = v.as_object().unwrap();
                        let price:f64 = price_obj.get("price").unwrap().as_str().unwrap().parse().unwrap();
                        best_price = price;
                        let new_price = wallet_balance * price;
                        new_total_balance += new_price;
                        new_total_equity += new_price;
                    }
                }

                let cross_un_pnl: f64 = obj.get("crossUnPnl").unwrap().as_str().unwrap().parse().unwrap();
                let pnl = cross_un_pnl + wallet_balance;
                new_total_balance += wallet_balance;
                new_total_equity += pnl;
            }
        }
        // 余额
        let total_wallet_balance: f64 = ((new_total_balance / best_price) - 34.27754) * best_price;
        // 权益
        let new_total_equity_eth: f64 = ((new_total_equity / best_price) - 34.27754) * best_price;
        // let total_balance: f64 = value
        //     .as_object()
        //     .unwrap()
        //     .get("totalWalletBalance")
        //     .unwrap()
        //     .as_str()
        //     .unwrap()
        //     .parse()
        //     .unwrap();
        // 可用余额
        let available_balance: f64 = value
            .as_object()
            .unwrap()
            .get("availableBalance")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        // let total_equity = total_balance + total_pnl; // 权益 = 余额 + 未实现盈亏
        
        // let total_margin: f64 = value
        //     .as_object()
        //     .unwrap()
        //     .get("totalMarginBalance")
        //     .unwrap()
        //     .as_str()
        //     .unwrap()
        //     .parse()
        //     .unwrap();
        // let total_marign_eth: f64 = total_margin / best_price;
        // let available_margin: f64 = value
        //     .as_object()
        //     .unwrap()
        //     .get("totalMaintMargin")
        //     .unwrap()
        //     .as_str()
        //     .unwrap()
        //     .parse()
        //     .unwrap();
        // let available_margin_eth: f64 = available_margin / best_price;
        // let locked_margin = total_margin - available_margin;
        // let locked_margin_eth: f64 = locked_margin / best_price;
        let positions = value.as_object().unwrap().get("positions").unwrap().as_array().unwrap();
        // let mut position: f64 = 0.0;
        let mut amts: f64 = 0.0;
        let mut prices: f64 = 0.0;

        // let mut short_position: f64 = 0.0;
        for p in positions {
            let obj = p.as_object().unwrap();
            let position_amt: f64 = obj.get("positionAmt").unwrap().as_str().unwrap().parse().unwrap();
            
            if position_amt == 0.0 {
                continue;
            } else {
                
            let symbol = obj.get("symbol").unwrap().as_str().unwrap();
            let symbols= &symbol[0..symbol.len()-4];
            // println!("symbols: {},symbol: {}", symbols, symbol);
            let sbol = format!("{}USDT", symbols);
            // println!("传过去的参数{}", sbol);
                if let Some(data) = http_api.get_klines(&sbol).await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    let price_obj = v.as_object().unwrap();

                    let price:f64 = price_obj.get("price").unwrap().as_str().unwrap().parse().unwrap();
                    // let new_amt = position_amt * price;
                    amts += position_amt;
                    prices = price;
                }
            }

        }
        let position = amts * prices;


        let leverage = position.abs() / new_total_equity; // 杠杆率 = 仓位价值 / 本金（账户总金额 + 未实现盈亏）
        // println!("当前杠杆率{}", leverage);
        let leverage_eth = position / total_wallet_balance;

        if let Some(data) = http_api.get_open_orders().await {
            let value: Value = serde_json::from_str(&data).unwrap();
            let open_orders = value.as_array().unwrap();
            let open_order = open_orders.len();

            println!("当前挂单数量:{}, name:{}", open_order, name);

           

            return Some(Sub {
                id: String::from(id.to_string()),
                name: String::from(name),
                total_balance_u:format!("{}", new_total_balance),
                total_balance: format!("{}", total_wallet_balance),
                total_equity: format!("{}", new_total_equity),
                total_equity_eth: format!("{}", new_total_equity_eth),
                leverage: format!("{}", leverage),
                leverage_eth: format!("{}", leverage_eth),
                position: format!("{}", position),
                open_order_amt: format!("{}", open_order),
                // day_transaction_price: format!("{}", day_transaction_price),
                // week_transaction_price: format!("{}", week_transaction_price),
                // day_pnl: format!("{}", day_pnl ),
                // week_pnl: format!("{}", week_pnl ),
                available_balance: format!("{}", available_balance),
            });
        } else {
            error!("Can't get {} openOrders.", name);
        return None;
            
        }
    } else {
        error!("Can't get {} account.", name);
        return None;
    }
}



// 获取仓位明细
pub async fn get_account_positions(
    http_api: &Box<dyn HttpVenueApi>,
    name: &str,
    id: &u64,
    origin_balance: f64,
) -> Vec<Value> {
    let mut history_positions: VecDeque<Value> = VecDeque::new();
    if let Some(data) = http_api.account().await {
        let value: Value = serde_json::from_str(&data).unwrap();
        // let mut history_positions: Vec<http_data::Position> = Vec::new();
        
        let positions = value.as_object().unwrap().get("positions").unwrap().as_array().unwrap();
        for p in positions {
            let mut pos_obj: Map<String, Value> = Map::new();
            let obj = p.as_object().unwrap();
            let amt:f64= obj.get("positionAmt").unwrap().as_str().unwrap().parse().unwrap();
            if amt == 0.0 {
                continue;
            } else {
                let symbol = obj.get("symbol").unwrap().as_str().unwrap();
            let millis = obj.get("updateTime").unwrap().as_i64().unwrap();
            let datetime: DateTime<Utc> = DateTime::from_utc(
                NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                Utc,
            );
            let position_amt= obj.get("positionAmt").unwrap().as_str().unwrap();
            // info!("datetime: {}", datetime);
            let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
            let position_side = obj.get("positionSide").unwrap().as_str().unwrap();
            let entry_price = obj.get("entryPrice").unwrap().as_str().unwrap();
            let leverage = obj.get("leverage").unwrap().as_str().unwrap();
            let mark_price = obj.get("initialMargin").unwrap().as_str().unwrap();
            let unrealized_profit = obj.get("unrealizedProfit").unwrap().as_str().unwrap();

            pos_obj.insert(String::from("symbol"), Value::from(symbol));
            pos_obj.insert(String::from("position_amt"), Value::from(position_amt));
            pos_obj.insert(String::from("time"), Value::from(time));
            pos_obj.insert(String::from("position_side"), Value::from(position_side));
            pos_obj.insert(String::from("entry_price"), Value::from(entry_price));
            pos_obj.insert(String::from("leverage"), Value::from(leverage));
            pos_obj.insert(String::from("mark_price"), Value::from(mark_price));
            pos_obj.insert(String::from("unrealized_profit"), Value::from(unrealized_profit));
            // 新加的
            pos_obj.insert(String::from("id"), Value::from(id.to_string()));

            history_positions.push_back(Value::from(pos_obj));
            }
        }
            return history_positions.into();
    } else {
        error!("Can't get {} account.", name);
        return history_positions.into();
    }
}

// 获取挂单明细
pub async fn get_open_orders(
    http_api: &Box<dyn HttpVenueApi>,
    name: &str,
    id: &u64,
    origin_balance: f64,
) -> Vec<Value> {
    let mut history_open_orders: VecDeque<Value> = VecDeque::new();
    if let Some(data) = http_api.get_open_orders().await {
        let value: Value = serde_json::from_str(&data).unwrap();
        // let mut history_positions: Vec<http_data::Position> = Vec::new();
        
        let open_orders = value.as_array().unwrap();
        if open_orders.len() == 0 {
            println!("当前没有挂单")
        } else {
            for a in open_orders {
                let obj = a.as_object().unwrap();
                let mut open_order_object: Map<String, Value> = Map::new();
                let millis = obj.get("time").unwrap().as_i64().unwrap();
                let datetime: DateTime<Utc> = DateTime::from_utc(
                    NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                    Utc,
                );
                // info!("datetime: {}", datetime);
                let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                
                let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                let r#type = obj.get("type").unwrap().as_str().unwrap();
                let mut type_value = "";
                if r#type == "LIMIT" {
                    type_value = "限价单"
                } else if r#type == "MARKET" {
                    type_value = "市价单"
                } else if r#type == "STOP" {
                    type_value = "止损限价单"
                } else if r#type == "STOP_MARKET" {
                    type_value = "止盈市价单"
                } else if r#type == "TAKE_PROFIT" {
                    type_value = "止盈限价单"
                } else if r#type == "TAKE_PROFIT_MARKET" {
                    type_value = "止盈市价单"
                } else if r#type == "TRAILING_STOP_MARKET" {
                    type_value = "跟踪止损单" 
                }
                let side = obj.get("side").unwrap().as_str().unwrap();
                let price = obj.get("price").unwrap().as_str().unwrap();
                let orig_qty = obj.get("origQty").unwrap().as_str().unwrap();
                let executed_qty = obj.get("executedQty").unwrap().as_str().unwrap();
                let reduce_only = obj.get("reduceOnly").unwrap().as_bool().unwrap();
                open_order_object.insert(String::from("time"), Value::from(time.clone()));
                open_order_object.insert(String::from("name"), Value::from(name));
                open_order_object.insert(String::from("symbol"), Value::from(symbol));
                open_order_object.insert(String::from("type"), Value::from(type_value));
                open_order_object.insert(String::from("side"), Value::from(side));
                open_order_object.insert(String::from("price"), Value::from(price));
                open_order_object.insert(String::from("orig_qty"), Value::from(orig_qty));
                open_order_object.insert(String::from("executed_qty"), Value::from(executed_qty));
                open_order_object.insert(String::from("reduce_only"), Value::from(reduce_only));
                history_open_orders.push_back(Value::from(open_order_object));
                // println!("11111{}", vec[a]);
            }
        }
            return history_open_orders.into();
    } else {
        error!("Can't get {} account.", name);
        return history_open_orders.into();
    }
}

// 获取资产明细
pub async fn get_history_accounts(
    http_api: &Box<dyn HttpVenueApi>,
    name: &str,
    id: &u64,
    origin_balance: f64,
) -> Vec<Value> {
    let mut history_assets: VecDeque<Value> = VecDeque::new();
    if let Some(data) = http_api.account().await {
        let value: Value = serde_json::from_str(&data).unwrap();
        // let mut history_positions: Vec<http_data::Position> = Vec::new();
        
        let assets = value.as_object().unwrap().get("assets").unwrap().as_array().unwrap();
        for p in assets {
            let mut asset_obj: Map<String, Value> = Map::new();
            let obj = p.as_object().unwrap();
            let amt:f64= obj.get("walletBalance").unwrap().as_str().unwrap().parse().unwrap();
            if amt == 0.0 {
                continue;
            } else {
                let symbol = obj.get("asset").unwrap().as_str().unwrap();
            let wallet_balance= obj.get("walletBalance").unwrap().as_str().unwrap();
            let unrealized_profit = obj.get("unrealizedProfit").unwrap().as_str().unwrap();
            let margin_balance = obj.get("marginBalance").unwrap().as_str().unwrap();
            let available_balance = obj.get("availableBalance").unwrap().as_str().unwrap();

            asset_obj.insert(String::from("symbol"), Value::from(symbol));
            asset_obj.insert(String::from("wallet_balance"), Value::from(wallet_balance));
            asset_obj.insert(String::from("unrealized_profit"), Value::from(unrealized_profit));
            asset_obj.insert(String::from("margin_balance"), Value::from(margin_balance));
            asset_obj.insert(String::from("availableBalance"), Value::from(available_balance));
            // 新加的
            asset_obj.insert(String::from("id"), Value::from(id.to_string()));

            history_assets.push_back(Value::from(asset_obj));
            }
        }
            return history_assets.into();
    } else {
        error!("Can't get {} account.", name);
        return history_assets.into();
    }
}


// 获取划转明细
pub async fn get_income_data(
    http_api: &Box<dyn HttpVenueApi>,
    name: &str,
    id: &u64,
) -> Vec<Value>{
    
    let mut trade_incomes: VecDeque<Value> = VecDeque::new();

    println!("传过来的数据,  name:{:?}, id:{:?}", name, id);
    let dt = Local::now().timestamp_millis();
    let last_day = dt - 1000*60*60*24;
    let mut day_amount = 0.0;
    let mut week_amount = 0.0;
    println!("当前时间戳{}", dt);

        if let Some(data) = http_api.get_income().await {
            let value: Value = serde_json::from_str(&data).unwrap();
            // println!("获取基金流水{:?}", value);
            if value["total"] != 0 {
                let income = value["rows"].as_array().unwrap();
            // let last_day = dt - 1000*60*4;
            for i in income {
                let mut income_obj: Map<String, Value> = Map::new();
                let obj = i.as_object().unwrap(); // positionAmt positionSide
                
                let status = obj.get("status").unwrap().as_str().unwrap();
                if status == "CONFIRMED" {
                    let time = obj.get("timestamp").unwrap().as_i64().unwrap();
                    let amount:f64 = obj.get("amount").unwrap().as_str().unwrap().parse().unwrap();
                    let asset = obj.get("asset").unwrap().as_str().unwrap();
                    week_amount += amount;
                    if time >= last_day {
                        day_amount += amount;
                    }
                    income_obj.insert(String::from("day_amount"), Value::from(day_amount.to_string()));
                    income_obj.insert(String::from("week_amount"), Value::from(week_amount.to_string()));
                    income_obj.insert(String::from("name"), Value::from(name));
                    income_obj.insert(String::from("id"), Value::from(id.to_string()));
                    income_obj.insert(String::from("time"), Value::from(time));
                    income_obj.insert(String::from("amount"), Value::from(amount));
                    income_obj.insert(String::from("asset"), Value::from(asset));
                    trade_incomes.push_back(Value::from(income_obj));
                } else {
                    continue;
                }  
            }
                
        }
            // println!("处理之后的账户资金账户数据{:?}", trade_incomes);
            return Vec::from(trade_incomes.clone());
        } else {
            error!("Can't get {} income.", name);
            return Vec::from(trade_incomes.clone());
        }
}






