use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: String,
}