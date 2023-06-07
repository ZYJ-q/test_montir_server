use async_trait::async_trait;

#[async_trait]
pub trait HttpVenueApi: Sync+Send {
    async fn account(&self) -> Option<String>;
    async fn position_risk(&self) -> Option<String>;
    async fn trade_hiostory(&self, symbol: &str) -> Option<String>;
    async fn position(&self) -> Option<String>;
    async fn get_klines(&self, symbol: &str) -> Option<String>;
    async fn get_income(&self) -> Option<String>;
    async fn get_open_orders(&self) -> Option<String>;
}
