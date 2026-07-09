use crate::endpoint::QueryType;

use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct AvInfo {
    symbol: String,
    name: String,
    currency: String,
    exchangeFullName: String,
    exchange: String,
}

pub struct AvEndpoint {
    pub key: String,
}

impl Default for AvEndpoint {
    fn default() -> Self {
        Self { key: String::new() }
    }
}

impl AvEndpoint {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }

    pub fn with_key(mut self, key: String) -> Self {
        self.key = key;
        self
    }
    pub fn get_endpoint(&self, endpoint: QueryType) -> String {
        match endpoint {
            QueryType::Info(symbol) => {
                format!(
                    "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
                    symbol, self.key
                )
            }
            QueryType::Quote(symbol) => {
                let entitlement = String::from("delayed"); // "realtime" if subscriptuin allows
                format!(
                    "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&entitlement={}&symbol={}&apikey={}",
                    entitlement, symbol, self.key
                )
            }
            QueryType::HistoricalData(symbol, from, to) => {
                format!(
                    "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
                    symbol, self.key
                )
            }
        }
    }
}
