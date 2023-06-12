use std::collections::HashMap;

use actix_web::web;
use futures_util::io::Read;
use log4rs::filter::threshold;
use mysql::prelude::*;
use mysql::*;

// use crate::common;

// use super::AlarmUnit;
use super::db_data::{Account, Active, Product, Trader, Trade, Position, NetWorth, Equity, NewPrice, HistoryIncomes, OpenOrders, PositionsAlarm};
use super::http_data::SignInProRes;

pub fn create_pool(config_db: HashMap<String, String>) -> Pool {
    let user = config_db.get("user").unwrap();
    let password = config_db.get("password").unwrap();
    let host = config_db.get("host").unwrap();
    let port = config_db.get("port").unwrap();
    let database = config_db.get("database").unwrap();
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );
    let pool = Pool::new(url).unwrap();
    return pool;
}

pub fn check_account(pool: web::Data<Pool>, name: &str, password: &str) -> Result<Option<Account>> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn
        .exec_first(
            r"select * from accounts where acc_name = :name and acc_password = :password",
            params! {
                "name" => name,
                "password" => password
            },
        )
        .map(
            // Unpack Result
            |row| {
                row.map(|(acc_id, acc_name, acc_password)| Account {
                    acc_id,
                    acc_name,
                    acc_password,
                })
            },
        );

    return res;
}

pub fn add_active(
    pool: web::Data<Pool>,
    account_id: u64,
    token: &str,
    name: &str,
) -> Result<Vec<SignInProRes>> {
    let mut conn = pool.get_conn().unwrap();
    let mut re: Vec<SignInProRes> = Vec::new();
    let res = conn
        .exec_first(
            r"select * from actives where name = :name",
            params! {
                "name" => name
            },
        )
        .map(
            // Unpack Result
            |row| {
                row.map(|(acc_id, token, name)| Active {
                    acc_id,
                    token,
                    name,
                })
            },
        );
    match res {
        Ok(resq) => match resq {
            Some(active) => {
                conn.exec_drop(
                    r"delete from actives where name = :name",
                    params! {
                        "name" => active.name
                    },
                )
                .unwrap();
            }
            None => {}
        },
        Err(_) => {}
    }

    let res = conn.exec_drop(
        r"INSERT INTO actives (acc_id, token, name) VALUES (:acc_id, :token, :name)",
        params! {
            "acc_id" => account_id,
            "token" => token,
            "name" => name,
        },
    );
    match res {
        Ok(()) => match get_products(pool, account_id) {
            Ok(res) => match res {
                Some(data) => {
                    for item in data {
                        re.push(SignInProRes {
                            name: String::from(item.prod_name),
                            id: item.prod_id.to_string(),
                        });
                    }
                    return Ok(re);
                }
                None => {
                    return Ok(re);
                }
            },
            Err(e) => {
                return Err(e);
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn is_active(pool: web::Data<Pool>, token: &str) -> bool {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"select * from actives where token = :token",
        params! {
            "token" => token,
        },
    );
    match res {
        Ok(()) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

pub fn remove_active(pool: web::Data<Pool>, name: &str, token: &str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"delete from actives where token = :token and name = :name",
        params! {
            "token" => token,
            "name" => name
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn get_products(pool: web::Data<Pool>, account_id: u64) -> Result<Option<Vec<Product>>> {
    let mut products: Vec<Product> = Vec::new();
    let mut conn = pool.get_conn().unwrap();
    let res: Result<Vec<u64>> = conn.exec(
        r"select prod_id from test_acc_prod where acc_id = :acc_id",
        params! {
            "acc_id" => account_id
        },
    );
    match res {
        Ok(prod_ids) => {
            for prod_id in prod_ids {
                let mut conn = pool.get_conn().unwrap();
                let prod = conn
                    .exec_first(
                        r"select * from test_products where prod_id = :prod_id",
                        params! {
                            "prod_id" => prod_id
                        },
                    )
                    .map(
                        // Unpack Result
                        |row| {
                            row.map(|(prod_id, prod_name, weixin_id, prog_id)| Product {
                                prod_id,
                                prod_name,
                                weixin_id,
                                prog_id,
                            })
                        },
                    );
                match prod {
                    Ok(produc) => match produc {
                        Some(product) => {
                            products.push(product);
                        }
                        None => {
                            continue;
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            return Ok(Some(products));
        }
        Err(e) => return Err(e),
    }
}

pub fn get_traders(pool: web::Data<Pool>, prod_id: &str) -> Result<HashMap<String, Trader>> {
    let mut traders: HashMap<String, Trader> = HashMap::new();
    let mut conn = pool.get_conn().unwrap();
    let res: Result<Vec<u64>> = conn.exec(
        r"select tra_id from test_prod_tra where prod_id = :prod_id",
        params! {
            "prod_id" => prod_id
        },
    );
    match res {
        Ok(ids) => {
            for tra_id in ids {
                let res = conn
                    .exec_first(
                        r"select * from test_traders where tra_id = :tra_id",
                        params! {
                            "tra_id" => tra_id
                        },
                    )
                    .map(
                        // Unpack Result
                        |row| {
                            row.map(
                                |(
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold
                                )| Trader {
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold

                                },
                            )
                        },
                    );
                match res {
                    Ok(trader) => match trader {
                        Some(tra) => {
                            traders.insert(format!("{}", &tra.name), tra);
                        }
                        None => {
                            continue;
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(traders);
}

pub fn test_get_traders(pool: web::Data<Pool>, prod_id: &str) -> Result<Vec<Trader>> {
    let mut traders: Vec<Trader> = Vec::new();
    let mut conn = pool.get_conn().unwrap();
    let res: Result<Vec<u64>> = conn.exec(
        r"select tra_id from test_prod_tra where prod_id = :prod_id",
        params! {
            "prod_id" => prod_id
        },
    );
    match res {
        Ok(ids) => {
            for tra_id in ids {
                let res = conn
                    .exec_first(
                        r"select * from test_traders where tra_id = :tra_id",
                        params! {
                            "tra_id" => tra_id
                        },
                    )
                    .map(
                        // Unpack Result
                        |row| {
                            row.map(
                                |(
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold
                                )| Trader {
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold

                                },
                            )
                        },
                    );
                match res {
                    Ok(trader) => match trader {
                        Some(tra) => {
                            traders.push(tra);
                        }
                        None => {
                            continue;
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(traders);
}




// 获取所有的api Key 数据（账户历史划转记录）
pub fn get_trader_incomes(pool: web::Data<Pool>) -> Result<HashMap<String, Trader>> {
    let mut incomes: HashMap<String, Trader> = HashMap::new();
    let mut conn = pool.get_conn().unwrap();
    let res = conn.query_map(
        "select * from test_traders",
        |(tra_id, tra_venue, ori_balance, tra_currency, api_key, secret_key, other_keys, r#type, name, show, threshold)| {
            Trader{ tra_id, tra_venue, ori_balance, tra_currency, api_key, secret_key, other_keys, r#type, name, show, threshold }
        }
        ).unwrap(); 

        for i in res {
            let name = i.name.as_str();
            incomes.insert(String::from(name), i);
        }
                
        // match res {
        //     Ok(trader) => match trader {
        //         Some(tra) => {
        //             incomes.insert(format!("{}_{}", &tra.name, &tra.tra_id), tra);
        //         }
        //         None => {
        //             return Ok(incomes);
        //         }
        //     },
        //     Err(e) => {
        //         return Err(e);
        //     }
        // }
    println!("incomes账户划转{:?}", incomes);
    return Ok(incomes);
}


// 获取账户划转记录

pub fn get_history_incomes(
    pool: web::Data<Pool>,
    tra_id: &str
) -> Result<Vec<HistoryIncomes>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
    if tra_id == "account1" {
        let incomes = conn.query_map(
            "select * from incomes order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
        println!("获取划转记录account1{:?}", incomes);
        return Ok(incomes);
    } else if tra_id == "account2" {
        let incomes = conn.query_map(
            "select * from incomes_2 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account2{:?}", incomes);
        return Ok(incomes);

        
    } else if tra_id == "account3" {
        let incomes = conn.query_map(
            "select * from incomes_3 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account3{:?}", incomes);
        return Ok(incomes);

    } else if tra_id == "account5" {
        let incomes = conn.query_map(
            "select * from incomes_4 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account4{:?}", incomes);
        return Ok(incomes);

    } else if tra_id == "account6" {
        let incomes = conn.query_map(
            "select * from incomes_5 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account5{:?}", incomes);
        return Ok(incomes);

    } else if tra_id == "account7" {
        let incomes = conn.query_map(
            "select * from incomes_6 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account5{:?}", incomes);
        return Ok(incomes);

    } else{
        let incomes = conn.query_map(
            "select * from incomes_7 order by time desc",
            |(time, r#type, asset, amount, tran_id, status)| {
                HistoryIncomes{ time, r#type, asset, amount, tran_id, status }
            }
            ).unwrap();
            println!("获取划转记录account6{:?}", incomes);
        return Ok(incomes);

    }
}

    
// 获取持仓数据和挂单数据和账户资产明细

pub fn get_trader_positions(pool: web::Data<Pool>, tra_id: &str) -> Result<HashMap<String, Trader>> {
    let mut traders: HashMap<String, Trader> = HashMap::new();
    let mut conn = pool.get_conn().unwrap();
    let res = conn
    .exec_first(
                r"select * from test_traders where tra_id = :tra_id",
                params! {
                        "tra_id" => tra_id
                        },
                )
                .map(
                        // Unpack Result
                        |row| {
                            row.map(
                                |(
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold
                                )| Trader {
                                    tra_id,
                                    tra_venue,
                                    ori_balance,
                                    tra_currency,
                                    api_key,
                                    secret_key,
                                    other_keys,
                                    r#type,
                                    name,
                                    show,
                                    threshold
                                },
                            )
                        },
                    );
                    match res {
                        Ok(trader) => match trader {
                            Some(tra) => {
                                traders.insert(format!("{}_{}", &tra.name, &tra.tra_id), tra);
                            }
                            None => {
                                return Ok(traders);
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
    println!("history_trader{:?}", traders);
    return Ok(traders);
}

// 获取历史交易数据

pub fn get_history_trades(
    pool: web::Data<Pool>,
    tra_id: &str
) -> Result<Vec<Trade>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
    if tra_id == "account1" {
        let trades = conn.query_map(
            "select * from trade_histories order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, is_maker, qty, quote_qty, tra_time, side, price, position_side, tra_commision, realized_pnl)| {
                Trade{th_id, tra_symbol, tra_order_id, is_maker, qty, quote_qty, tra_time, side, price, position_side, tra_commision, realized_pnl}
            }
            ).unwrap();
        // println!("获取历史交易数据account1{:?}", trades);
        return Ok(trades);
    } else if tra_id == "account3" {
        let trades = conn.query_map(
            "select * from trade_histories_3 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);

        
    } else if tra_id == "account4" {
        let trades = conn.query_map(
            "select * from trade_histories_4 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else if tra_id == "account5" {
        let trades = conn.query_map(
            "select * from trade_histories_5 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else if tra_id == "account7" {
        let trades = conn.query_map(
            "select * from trade_histories_7 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else if tra_id == "account8" {
        let trades = conn.query_map(
            "select * from trade_histories_8 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else if tra_id == "account9" {
        let trades = conn.query_map(
            "select * from trade_histories_9 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else if tra_id == "account10" {
        let trades = conn.query_map(
            "select * from trade_histories_10 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);

    } else {
        let trades = conn.query_map(
            "select * from trate_histories_2 order by tra_time desc limit 1000",
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account2{:?}", trades);
        return Ok(trades);
    }
}

// 获取持仓数据
pub fn get_history_positions(
    pool: web::Data<Pool>,
    tra_id: &str
) -> Result<Vec<Position>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
    if tra_id == "account1" {
        let positions = conn.query_map(
            "select * from position_histories order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account1{:?}", positions);
        return Ok(positions);
    } else if tra_id == "account3" {
        let positions = conn.query_map(
            "select * from position_histories_3 order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account3{:?}", positions);
        return Ok(positions);
        
    } else if tra_id == "account4" {
        let positions = conn.query_map(
            "select * from position_histories_4 order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account2{:?}", positions);
        return Ok(positions);
        
    } else if tra_id == "account5" {
        let positions = conn.query_map(
            "select * from position_histories_5 order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account2{:?}", positions);
        return Ok(positions);
        
    } else if tra_id == "account6" {
        let positions = conn.query_map(
            "select * from position_histories_6 order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account2{:?}", positions);
        return Ok(positions);
        
    } else {
        let positions = conn.query_map(
            "select * from position_histories_2 order by time desc",
            |(symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price)| {
                Position{symbol, position_amt, position_side, time, entry_price, un_realized_profit, tra_id, leverage, mark_price}
            }
            ).unwrap();
        // println!("获取历史仓位数据account2{:?}", positions);
        return Ok(positions);
    }
    
}

// 获取净值数据
pub fn get_history_networth(
    pool: web::Data<Pool>
) -> Result<Vec<NetWorth>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
        let net_worths = conn.query_map(
            "select * from net_worth",
            |(time, net_worth)| {
                NetWorth{ time, net_worth }
            }
            ).unwrap();
        // println!("获取历史净值数据{:?}", net_worths);
        return Ok(net_worths);
}

// 获取权益数据（计算盈亏）
// 获取净值数据
pub fn get_equity(
    pool: web::Data<Pool>
) -> Result<Vec<Equity>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
        let equitys = conn.query_map(
            "select * from (select * from equity order by id desc limit 12) tbl1 order by id limit 7;",
            |(id, name, time, equity_eth, equity, prod_id)| {
                Equity{id, name, time, equity_eth, equity, prod_id }
            }
            ).unwrap();
        println!("获取历史净值数据{:?}", equitys);
        return Ok(equitys);
}

// 获取账户交易额
pub fn get_trade_price(
    pool: web::Data<Pool>
) -> Result<Vec<NewPrice>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
        let net_worths = conn.query_map(
            "select * from trade_price",
            |(id, name, week_price, day_price)| {
                NewPrice{id, name, week_price, day_price }
            }
            ).unwrap();
        // println!("获取历史净值数据{:?}", net_worths);
        return Ok(net_worths);
}


// 根据日期获取账户交易历史的数据
pub fn get_date_history_trades(
    pool: web::Data<Pool>,
    start_time: &str,
    end_time: &str,
    tra_id: &str
) -> Result<Vec<Trade>> {
    let mut conn = pool.get_conn().unwrap();
    // let mut re: Vec<Trade> = Vec::new();
    if tra_id == "account1" {
       let value = &format!("select * from trade_histories where tra_time >= {} and tra_time <= {}", start_time, end_time);
       let trades = conn.query_map(
        value,
        |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
            Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
        }
        ).unwrap();
    // println!("获取历史交易数据account3{:?}", trades);
    return Ok(trades);
    } else if tra_id == "Angus" {
        let value = &format!("select * from trade_histories_3 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        println!("获取历史交易数据angus{:?}", trades);
        return Ok(trades);
    } else if tra_id == "trader02" {
        let value = &format!("select * from trade_histories_4 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else if tra_id == "trader04" {
        let value = &format!("select * from trade_histories_5 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else if tra_id == "xh01_feng4_virtual" {
        let value = &format!("select * from trade_histories_7 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else if tra_id == "xh02_b20230524_virtual" {
        let value = &format!("select * from trade_histories_8 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else if tra_id == "xh03_feng3_virtual" {
        let value = &format!("select * from trade_histories_9 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else if tra_id == "xh04_20230524_virtual" {
        let value = &format!("select * from trade_histories_10 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    } else {
        let value = &format!("select * from trade_histories_2 where tra_time >= {} and tra_time <= {}", start_time, end_time);
        let trades = conn.query_map(
            value,
            |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
                Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
            }
            ).unwrap();
        // println!("获取历史交易数据account3{:?}", trades);
        return Ok(trades);
    }
    
}

// 获取所有的产品列表
pub fn get_all_products(pool: web::Data<Pool>) -> Result<Vec<Product>> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.query_map(
        r"select * from test_products",
        |(prod_id, prod_name, weixin_id, prog_id)| {
            Product{ prod_id, prod_name, weixin_id, prog_id }
        }
    ).unwrap();
    return Ok(res);
}

// 获取挂单监控列表
pub fn get_alarm_open_orders(pool: web::Data<Pool>) -> Result<Vec<OpenOrders>> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.query_map(
        r"select * from open_orders",
        |(id, api_key, secret_key, name)| {
            OpenOrders{ id, api_key, secret_key, name }
        }
    ).unwrap();
    return Ok(res);
}

// 删除挂单监控
pub fn delect_orders(pool: web::Data<Pool>, name:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"delete from open_orders where name = :name",
        params! {
            "name" => name
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 添加挂单
pub fn add_orders(pool: web::Data<Pool>, name:&str, api_key: &str, secret_key:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"INSERT INTO open_orders (api_key, secret_key, name)
        VALUES (:api_key, :secret_key, :name)",
        params! {
            "api_key" => api_key,
            "secret_key" => secret_key,
            "name" => name
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 获取净头寸监控列表
pub fn get_alarm_positions(pool: web::Data<Pool>) -> Result<Vec<PositionsAlarm>> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.query_map(
        r"select * from position_alarm",
        |(id, api_key, secret_key, name, threshold)| {
            PositionsAlarm{ id, api_key, secret_key, name, threshold }
        }
    ).unwrap();
    return Ok(res);
}

// 删除净头寸监控
pub fn delect_positions(pool: web::Data<Pool>, name:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"delete from position_alarm where name = :name",
        params! {
            "name" => name
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 添加监控账号
pub fn add_positions(pool: web::Data<Pool>, name:&str, api_key: &str, secret_key:&str, threshold:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"INSERT INTO test (api_key, secret_key, name, threshold)
        VALUES (:api_key, :secret_key, :name, :threshold)",
        params! {
            "api_key" => api_key,
            "secret_key" => secret_key,
            "name" => name,
            "threshold" => threshold
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 更新净头寸监控中的阈值
pub fn update_positions(pool: web::Data<Pool>, name:&str, threshold:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"update test_traders set threshold = :threshold where name = :name",
        params! {
            "name" => name,
            "threshold" => threshold
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 设置账户的份额
pub fn update_ori_balance(pool: web::Data<Pool>, name:&str, ori_balance:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"update traders set ori_balance = :ori_balance where name = :name",
        params! {
            "name" => name,
            "ori_balance" => ori_balance
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 更新是否打开监控开关
pub fn update_alarms(pool: web::Data<Pool>, name:&str, alarm:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"update test_traders set alarm = :alarm where name = :name",
        params! {
            "name" => name,
            "alarm" => alarm
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 删除账户
pub fn delect_accounts(pool: web::Data<Pool>, name:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"delete from test_traders where name = :name",
        params! {
            "name" => name
        },
    );
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}


// 添加账户
pub fn add_accounts(pool: web::Data<Pool>, name:&str, api_key: &str, secret_key:&str, alarm:&str, threshold:&str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        r"INSERT INTO test_traders (tra_venue, ori_balance, tra_currency, api_key, secret_key, other_keys, type, name, alarm, threshold)
        VALUES (:tra_venue, :ori_balance, :tra_currency, :api_key, :secret_key, :other_keys, :type, :name, :alarm, :threshold)",
        params! {
            "tra_venue" => "Binance",
            "ori_balance" => "500",
            "tra_currency" => "USDT", 
            "api_key" => api_key,
            "secret_key" => secret_key,
            "other_keys" => "",
            "type" => "Futures",
            "name" => name,
            "alarm" => alarm,
            "threshold" => threshold
        },
    );


    
    match res {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}


// 查找tra_id并添加到test_prod_tra表中

pub fn select_id(pool: web::Data<Pool>, name: &str, prod_id: &str) -> Result<()> {
    let mut conn = pool.get_conn().unwrap();

    println!("传过来的参数{}", prod_id);

    let res:Result<Vec<u64>> = conn.exec(
        "select tra_id from test_traders where name = :name", 
        params! {
            "name" => name
        },
    );

    // println!("data数据数据数据数据{:?}", res);
    // match data {
    //     Ok(tra_id) => {
    //         println!("查询到的tra_id", tra_id);
    //         conn.exec(
    //             r"INSERT INTO tset_prod_tra (pt_id, prod_id, tra_id) VALUES (:pt_id, :prod_id, :tra_id)", 
    //             params! {
    //                 "prod_id" => prod_id,
    //                 "tra_id" => tra_id,
    //             },
    //         );
    //     }
    //     Err(_) => todo!(),
        
    // }

    
    match res {
        Ok(tra_id) => {
            println!("tra_id{:?}", tra_id);
            let _data = conn.exec_drop(
                r"INSERT INTO test_prod_tra (prod_id, tra_id) VALUES (:prod_id, :tra_id)", 
                params! {
                    "prod_id" => prod_id,
                    "tra_id" => tra_id[0]
                },
            );
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}
