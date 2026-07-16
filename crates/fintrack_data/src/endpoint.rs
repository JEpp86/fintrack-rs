use async_compat::Compat;
use reqwest;
use smol;

use crate::AvEndpoint;
use crate::FmpEndpoint;

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
        Self::get_request(&url)
    }

    fn get_av_endpoint(&self, key: &str, query: QueryType) -> String {
        let url = AvEndpoint::new(key).get_endpoint(query);
        Self::get_request(&url)
    }

    fn get_request(url: &str) -> String {
        println!("Endpoint: {url}");
        let result: Result<serde_json::Value, reqwest::Error> = smol::block_on(async {
            let response = Compat::new(reqwest::get(url))
                .await?
                .json::<serde_json::Value>()
                .await?;
            Ok(response.clone())
        });
        let result =
            result.unwrap_or_else(|_| serde_json::Value::String(String::from("API Request Error")));
        if result.is_array() {
            return serde_json::to_string_pretty(&result[0])
                .expect("Unable to format JSON response");
        }
        serde_json::to_string_pretty(&result).expect("Unable to format JSON response")
    }
}

pub enum QueryType {
    Info(String),
    Quote(String),
    HistoricalData(String, String, String),
}
