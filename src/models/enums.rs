use std::collections::HashMap;

use crate::actors::adapters::{
    base::venue_api::HttpVenueApi, binance::futures_api::BinanceFuturesApi,
};

#[derive(Clone)]
pub enum VenueApi {
    BinanceFuturesApi,
}

pub fn get_venue_api(
    venues: HashMap<String, VenueApi>,
    venue_type: &str,
    params: HashMap<String, &str>,
) -> Box<dyn HttpVenueApi> {
    match venues.get(venue_type) {
        Some(data) => match data {
            VenueApi::BinanceFuturesApi => {
                return Box::new(BinanceFuturesApi::new(
                    params.get("base_url").unwrap(),
                    params.get("api_key").unwrap(),
                    params.get("api_secret").unwrap(),
                ));
            }
        },
        None => {
            panic!("Didn't realize {} venue api yet.", venue_type);
        }
    }
}