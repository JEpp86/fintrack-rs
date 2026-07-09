// Implementation of the Financial Modelling Prep API endpoints
// URL: https://site.financialmodelingprep.com/

use crate::endpoint::QueryType;

use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct FmpInfo {
    symbol: String,
    name: String,
    currency: String,
    exchangeFullName: String,
    exchange: String,
}

pub struct FmpEndpoint {
    pub key: String,
}

impl Default for FmpEndpoint {
    fn default() -> Self {
        Self { key: String::new() }
    }
}

impl FmpEndpoint {
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
                    "https://financialmodelingprep.com/stable/profile?symbol={}&apikey={}",
                    symbol, self.key
                )
            }
            QueryType::Quote(symbol) => {
                format!(
                    "https://financialmodelingprep.com/stable/quote/{}?apikey={}",
                    symbol, self.key
                )
            }
            QueryType::HistoricalData(symbol, from, to) => {
                format!(
                    "https://financialmodelingprep.com/stable/historical-price-full/{}?from={}&to={}&apikey={}",
                    symbol, from, to, self.key
                )
            }
        }
    }
}
