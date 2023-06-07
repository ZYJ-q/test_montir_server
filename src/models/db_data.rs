use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub acc_id: u64,
    pub acc_name: String,
    pub acc_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccProd {
    pub ap_id: u64,
    pub acc_id: u64,
    pub prod_id: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub prod_id: u64,
    pub prod_name: String,
    pub weixin_id: u64,
    pub prog_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenOrders {
    pub id: u64,
    pub api_key: String,
    pub secret_key: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionsAlarm {
    pub id: u64,
    pub api_key: String,
    pub secret_key: String,
    pub name: String,
    pub threshold: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Active {
    pub acc_id: u64,
    pub token: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trader {
    pub tra_id: u64,
    pub tra_venue: String,
    pub ori_balance: String,
    pub tra_currency: String,
    pub api_key: String,
    pub secret_key: String,
    pub other_keys: String,
    pub r#type: String,
    pub name: String,
    pub show: String,
    pub threshold: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub th_id: u64,
    pub tra_symbol: String,
    pub tra_order_id: u64,
    // pub tra_id: u64,
    pub tra_commision: String,
    pub tra_time: u64,
    pub is_maker: String,
    pub position_side: String,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Incomes {
    pub tra_id: u64,
    pub tra_venue: String,
    pub ori_balance: String,
    pub tra_currency: String,
    pub api_key: String,
    pub secret_key: String,
    pub other_keys: String,
    pub r#type: String,
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryIncomes {
    pub time: String,
    pub r#type: String,
    pub asset: String,
    // pub tra_id: u64,
    pub amount: String,
    pub tran_id: u64,
    pub status: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub symbol: String, 
    pub position_amt: String, 
    pub position_side: String, 
    pub time: String, 
    pub entry_price: String, 
    pub un_realized_profit: String, 
    pub tra_id: u64, 
    pub leverage: String, 
    pub mark_price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetWorth {
    pub time: String,
    pub net_worth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Equity {
    pub id: u64,
    pub name: String,
    pub time: String,
    pub equity_eth: String,
    pub equity: String,
    pub prod_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPrice {
    pub id: u64,
    pub name: String,
    pub week_price: String,
    pub day_price: String,
}