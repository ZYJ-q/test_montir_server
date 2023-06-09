use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignIn {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignInProRes {
    pub(crate) name: String,
    pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignInRes {
    pub(crate) name: String,
    pub(crate) products: Vec<SignInProRes>,
    pub(crate) token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TradeRe {
    pub th_id: u64,
    pub tra_symbol: String,
    pub tra_order_id: u64,
    // pub tra_id: u64,
    pub tra_commision: String,
    pub tra_time: String,
    pub is_maker: String,
    pub position_side: String,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TradeRes {
    pub trades_history: Vec<TradeRe>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SignOut {
    pub name: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub prod_id: String,
    pub r#type: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub tra_id: String,
    pub r#type: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTrade {
    pub tra_id: String,
    pub r#type: String,
    pub token: String,
    pub start_time: String,
    pub end_time: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equity {
    pub r#type: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelectOrders{
    pub r#type: String,
    pub token: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAccounts{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub api_key: String,
    pub secret_key: String,
    pub alarm: String,
    pub threshold: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectId{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub prod_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrders{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub api_key: String,
    pub secret_key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPositions{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub api_key: String,
    pub secret_key: String,
    pub threshold: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePositions{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub threshold: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlarms{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub alarm: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOriBalance{
    pub r#type: String,
    pub token: String,
    pub name: String,
    pub ori_balance: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Posr {
    pub tra_id: String,
    pub r#type: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomesRe {
    pub r#type: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetWorthRe {
    pub r#type: String,
    pub token: String
}







#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub time: String,
    pub entry_price: String,
    pub leverage: String,
    pub mark_price: String,
    pub position_amt: String,
    pub position_side: String,
    pub symbol: String, 
    pub un_realized_profit: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionsRe {
    pub positions: Vec<Position>,
}



#[derive(Serialize, Deserialize, Clone)]
pub struct Total {
    pub equity_eth: String,
    pub net_worth: String,
    pub net_worth_eth: String,
    pub equity: String,
    // pub day_pnl: String,
    // pub week_pnl: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sub {
    pub id: String,
    pub name: String,
    // pub origin_balance: String,
    pub total_balance_u: String,
    pub total_balance: String,
    // pub total_margin: String,
    // pub total_margin_eth: String, 
    pub total_equity: String,
    pub total_equity_eth: String,
    pub leverage: String,
    pub leverage_eth: String,
    pub open_order_amt: String,
    // pub available_margin: String,
    // pub available_margin_eth: String,
    // pub locked_margin: String,
    // pub locked_margin_eth: String,
    pub position: String,
    // pub short_position: String,
    // pub day_transaction_qty: String,
    // pub week_transaction_qty: String,
    // pub day_transaction_price: String,
    // pub week_transaction_price: String,
    // pub day_pnl: String,
    // pub week_pnl: String,
    pub available_balance: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountRe {
    pub total: Total,
    pub subs: Vec<Sub>,
}

impl AccountRe {
    pub fn new() -> Self {
        Self {
            total: Total {
                time: String::new(),
                equity_eth: String::new(),
                net_worth_eth: String::new(),
                net_worth: String::new(),
                equity: String::new(),
                // day_pnl: String::new(),
                // week_pnl: String::new(),
            },
            subs: Vec::new(),
        }
    }
}
