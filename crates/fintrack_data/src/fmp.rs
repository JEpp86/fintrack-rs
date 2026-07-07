// Implementation of the Financial Modelling Prep API endpoints
// URL: https://site.financialmodelingprep.com/

use crate::endpoint::{Endpoint, Endpoints, QueryType};

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

// impl Endpoint for FmpEndpoint {
//     fn get_info(&self, symbol: &str) -> Result<String, String> {
//         let url = self.get_endpoint(symbol, Endpoints::StockInfo);
//         let result: Result<serde_json::Value, reqwest::Error> = smol::block_on(async {
//             let response = Compat::new(reqwest::get(&url))
//                 .await?
//                 .json::<serde_json::Value>()
//                 .await?;
//             Ok(response[0].clone())
//         });
//         let op = serde_json::to_string_pretty(&result.expect("Unable to parse JSON response"))
//             .expect("Unable to format JSON response");
//         Ok(op)
//     }

//     fn get_historical_data(&self, symbol: &str, start: &str, end: &str) -> Result<String, String> {
//         let url = self.get_endpoint(
//             symbol,
//             Endpoints::HistoricalData(start.to_string(), end.to_string()),
//         );
//         let result: Result<serde_json::Value, reqwest::Error> = smol::block_on(async {
//             let response = Compat::new(reqwest::get(&url))
//                 .await?
//                 .json::<serde_json::Value>()
//                 .await?;
//             Ok(response[0].clone())
//         });
//         let op = serde_json::to_string_pretty(&result.expect("Unable to parse JSON response"))
//             .expect("Unable to format JSON response");
//         Ok(op)
//     }

//     fn get_quote(&self, symbol: &str) -> Result<String, String> {
//         let url = self.get_endpoint(symbol, Endpoints::Quote);
//         let result: Result<serde_json::Value, reqwest::Error> = smol::block_on(async {
//             let response = Compat::new(reqwest::get(&url))
//                 .await?
//                 .json::<serde_json::Value>()
//                 .await?;
//             Ok(response[0].clone())
//         });
//         let op = serde_json::to_string_pretty(&result.expect("Unable to parse JSON response"))
//             .expect("Unable to format JSON response");
//         Ok(op)
//     }
// }
