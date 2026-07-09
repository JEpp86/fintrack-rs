use async_compat::Compat;
use reqwest;
use smol;

use crate::AvEndpoint;
use crate::FmpEndpoint;

// pub struct Ticker {
//     pub symbol: String,
//     // key: String,
//     endpoint: Box<dyn Endpoint>,
// }

// pub trait Endpoint {
//     // fn get_endpoint(&self, endpoint: Endpoints) -> String;
//     fn get_info(&self, symbol: &str) -> Result<String, String>;
//     fn get_historical_data(&self, symbol: &str, start: &str, end: &str) -> Result<String, String>;
//     fn get_quote(&self, symbol: &str) -> Result<String, String>;
// }

pub enum Endpoints {
    FinancialModelPrep,
    AlphaVantage,
}

impl Endpoints {
    pub fn get_endpoint(&self, key: &str, query: QueryType) -> String {
        match self {
            Endpoints::FinancialModelPrep => self.get_fmp_endpoint(key, query),
            Endpoints::AlphaVantage => self.get_av_endpoint(key, query),
        }
    }

    fn get_fmp_endpoint(&self, key: &str, query: QueryType) -> String {
        let url = FmpEndpoint::new(key).get_endpoint(query);
        get_request(&url)
    }

    fn get_av_endpoint(&self, key: &str, query: QueryType) -> String {
        let url = AvEndpoint::new(key).get_endpoint(query);
        get_request(&url)
    }
}

pub enum QueryType {
    Info(String),
    Quote(String),
    HistoricalData(String, String, String),
}

// impl QueryType {
fn get_request(url: &str) -> String {
    let result: Result<serde_json::Value, reqwest::Error> = smol::block_on(async {
        let response = Compat::new(reqwest::get(url))
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(response[0].clone())
    });
    serde_json::to_string_pretty(&result.expect("Unable to parse JSON response"))
        .expect("Unable to format JSON response")
}
// }
