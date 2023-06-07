use log:: warn;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};

pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client: Client = Client::new();
        Self { client }
    }

    pub async fn send_request(
        &self,
        method: &str,
        url: &str,
        headers_r: HeaderMap,
        body: &str,
    ) -> Option<Response> {
        // env_logger::init();
        let max_try = 5;
        let mut try_count = 0;
        while try_count < max_try {
            let headers = headers_r.clone();
            if method == "GET" {
                match self.client.get(url).headers(headers).send().await {
                    Ok(response) => {
                        return Some(response);
                    }
                    Err(e) => {
                        warn!("{}, retry {}", e, try_count);
                        try_count += 1;
                        continue;
                    }
                }
            } else if method == "POST" {
                match self
                    .client
                    .post(url)
                    .headers(headers)
                    .body(String::from(body))
                    .send()
                    .await
                {
                    Ok(response) => {
                        return Some(response);
                    }
                    Err(e) => {
                        warn!("{}, retry {}", e, try_count);
                        try_count += 1;
                        continue;
                    }
                }
            } else if method == "DELETE" {
                match self.client.delete(url).headers(headers).send().await {
                    Ok(response) => {
                        return Some(response);
                    }
                    Err(e) => {
                        warn!("{}, retry {}", e, try_count);
                        try_count += 1;
                        continue;
                    }
                }
            }
        }
        return None;
    }
}
