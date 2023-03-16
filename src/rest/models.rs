use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: i32,
    pub market_data: Option<MarketData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub roi: Option<Roi>,
    pub market_cap: Option<PriceData>,
    pub fully_diluted_valuation: Option<PriceData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceData {
    pub usd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}