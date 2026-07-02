use std::io::Error;

pub struct Ticker {
    pub symbol: String,
    // key: String,
    endpoint: Box<dyn Endpoint>,
}

pub trait Endpoint {
    // fn get_endpoint(&self, endpoint: Endpoints) -> String;
    fn get_info(&self, symbol: &str) -> Result<String, String>;
    fn get_historical_data(&self, symbol: &str, start: &str, end: &str) -> Result<String, String>;
    fn get_quote(&self, symbol: &str) -> Result<String, String>;
}

pub enum Endpoints {
    StockInfo,
    Quote,
    HistoricalData(String, String),
}
