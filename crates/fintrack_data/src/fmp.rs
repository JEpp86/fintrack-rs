// Implementation of the Financial Modelling Prep API endpoints
// URL: https://site.financialmodelingprep.com/

use crate::ticker::{Endpoint, Endpoints};

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
                    "https://financialmodelingprep.com/stable/search-symbol?query={}&apikey={}",
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
        self.get_endpoint(symbol, Endpoints::StockInfo)
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
