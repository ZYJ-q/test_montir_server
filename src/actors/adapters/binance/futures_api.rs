use async_trait::async_trait;
use chrono::{Utc, Local};
use hex;
use hmac::{Hmac, Mac};
use itertools::Itertools;
use log::error;
use reqwest::header::HeaderMap;
use reqwest::{Method, Response, StatusCode};
use serde_json::value::Value;
use sha2::Sha256;
use std::collections::HashMap;

use super::http::HttpClient;
use super::base::venue_api::HttpVenueApi;

pub struct BinanceFuturesApi {
    client: HttpClient,
    base_url: String,
    api_key: String,
    api_secret: String,
}

impl BinanceFuturesApi {
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        let http_client = HttpClient::new();
        Self {
            client: http_client,
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
        }
    }

    async fn package_request(
        &self,
        method: &Method,
        url: &str,
        need_sign: bool,
        params: &HashMap<String, Value>,
    ) -> Option<Response> {
        // env_logger::init();
        let mut uri = String::from(url);
        let mut data_json = String::new();

        if method == Method::GET || method == Method::DELETE {
            if !params.is_empty() {
                let mut strl: Vec<String> = Vec::new();
                for key in params.keys().sorted() {
                    let value = params.get(key).unwrap();
                    if value.is_string() {
                        strl.push(format!("{}={}", key, value.as_str().unwrap()));
                    } else {
                        strl.push(format!("{}={}", key, value));
                    }
                }
                for i in 0..strl.len() {
                    if i == 0 {
                        data_json.push_str(&strl[i]);
                    } else {
                        data_json.push('&');
                        data_json.push_str(&strl[i]);
                    }
                }
                uri = format!("{}?{}", &uri, &data_json);
            }
        } else {
            if !params.is_empty() {
                match serde_json::to_string(&params) {
                    Ok(result) => data_json = result,
                    Err(e) => {
                        error!("error on parase params: {}", e);
                        return None;
                    }
                }
            }
        }

        let mut headers = HeaderMap::new();
        if need_sign {
            let str_to_sign = format!("{}", &data_json);
            // println!("{str_to_sign}");

            let mut hmac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes()).unwrap();
            hmac.update(str_to_sign.as_bytes());
            let sign_bytes = hmac.finalize().into_bytes();
            let sign = hex::encode(sign_bytes);

            uri = format!("{}&signature={}", &uri, &sign);

            headers.insert("X-MBX-APIKEY", self.api_key.parse().unwrap());
            headers.insert("Content-Type", "application/json".parse().unwrap());
        }
        headers.insert("User-Agent", "nautilus_alarm".parse().unwrap());
        let url = format!("{}{}", self.base_url, uri);
        // println!("{},{},{:?},{}", &method.as_str(), url, headers, data_json);
        return self
            .client
            .send_request(&method.as_str(), &url, headers, &data_json)
            .await;
    }

    async fn send(
        &self,
        method: Method,
        url: &str,
        need_sign: bool,
        params: &HashMap<String, Value>,
    ) -> Option<String> {
        // let data: HashMap<String, Value> = HashMap::new();
        if let Some(response) = self.package_request(&method, url, need_sign, params).await {
            if response.status() == StatusCode::OK {
                match response.text().await {
                    Ok(response_data) => {
                        return Some(response_data);
                    }
                    Err(e) => {
                        error!("error on parse response: {:?}", e);
                        return None;
                    }
                }
            } else {
                error!(
                    "code status error: {}-{}",
                    response.status(),
                    response.text().await.unwrap()
                );
                return None;
            }
        } else {
            error!(
                "none response: {},{},{},{:?}",
                &method.as_str(),
                url,
                need_sign,
                params
            );
            return None;
        }
    }

    // todo
    fn check_response_data(&self, data_s: Option<String>) -> Option<String> {
        match data_s {
            Some(data) => {
                if !data.is_empty() {
                    if data.contains("code") {
                        error!("code: {}", data);
                        return None;
                    } else {
                        return Some(data);
                    }
                } else {
                    error!("response is empty");
                    return None;
                }
            }
            None => {
                error!("handle response failed");
                return None;
            }
        }
    }
}

#[async_trait]
impl HttpVenueApi for BinanceFuturesApi {
    async fn account(&self) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .send(Method::GET, "/fapi/v2/account", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    async fn position_risk(&self) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    async fn trade_hiostory(&self, symbol: &str) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));
        params.insert(String::from("limit"), Value::from(1000));

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .send(Method::GET, "/fapi/v1/userTrades", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                // print!("成交历史数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    async fn position(&self) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
    async fn get_klines(&self, symbol: &str) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));

        // let now_time = Utc::now().timestamp_millis();
        // params.insert(String::from("interval"), Value::from("15m"));
        

        let response = self
            .send(Method::GET, "/fapi/v1/ticker/price", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                // print!("K线数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    // 获取当前挂单
    async fn get_open_orders(&self) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .send(Method::GET, "/fapi/v1/openOrders", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                // print!("K线数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

//    获取账户资金流水明细（其中把转账明细筛选出来）
    async fn get_income(&self) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();

        let dt = Local::now().timestamp_millis();
        let last_day = dt - 1000*60*60*24 * 8;

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));
        params.insert(String::from("startTime"), Value::from(last_day));

        let response = self
            .send(Method::GET, "/sapi/v1/futures/transfer", true, &mut params)
            .await;

        let res_data = self.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
}
