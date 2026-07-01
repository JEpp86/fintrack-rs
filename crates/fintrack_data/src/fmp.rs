// Implementation of the Financial Modelling Prep API endpoints
// URL: https://site.financialmodelingprep.com/

use std::backtrace;

use crate::ticker::{Endpoint, Endpoints};

use async_compat::Compat;
use reqwest;
use serde::Deserialize;
use serde_json;
use smol;

#[derive(Deserialize)]
struct FmpInfo {
    symbol: String,
    name: String,
    currency: String,
    exchangeFullName: String,
    exchange: String,
}

pub struct FmpEndpoint {
    //pub symbol: String,
    pub key: String,
}

impl Default for FmpEndpoint {
    fn default() -> Self {
        Self {
            //symbol: String::new(),
            key: String::new(),
        }
    }
}

impl FmpEndpoint {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn with_key(mut self, key: String) -> Self {
        self.key = key;
        self
    }
    fn get_endpoint(&self, symbol: &str, endpoint: Endpoints) -> String {
        match endpoint {
            Endpoints::StockInfo => {
                format!(
                    //"https://financialmodelingprep.com/stable/search-symbol?query={}&apikey={}",
                    "https://financialmodelingprep.com/stable/profile?symbol={}&apikey={}",
                    symbol, self.key
                )
            }
            Endpoints::Quote => {
                format!(
                    "https://financialmodelingprep.com/stable/quote/{}?apikey={}",
                    symbol, self.key
                )
            }
            Endpoints::HistoricalData(from, to) => {
                format!(
                    "https://financialmodelingprep.com/stable/historical-price-full/{}?from={}&to={}&apikey={}",
                    symbol, from, to, self.key
                )
            }
        }
    }
}

impl Endpoint for FmpEndpoint {
    fn get_info(&self, symbol: &str) -> String {
        let url = self.get_endpoint(symbol, Endpoints::StockInfo);
        let result: Result<String, reqwest::Error> = smol::block_on(async {
            let body = Compat::new(reqwest::get(&url))
                .await?
                .json::<serde_json::Value>()
                .await?;
            Ok(serde_json::to_string_pretty(&body[0]).unwrap_or_default())
        });
        result.unwrap_or_default()
    }

    fn get_historical_data(&self, symbol: &str, start: &str, end: &str) -> String {
        self.get_endpoint(
            symbol,
            Endpoints::HistoricalData(start.to_string(), end.to_string()),
        )
    }

    fn get_quote(&self, symbol: &str) -> String {
        self.get_endpoint(symbol, Endpoints::Quote)
    }
}
