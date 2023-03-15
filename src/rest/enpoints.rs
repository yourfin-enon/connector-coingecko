use super::{rest_client::CoingeckoRestClient};

pub enum CoingeckoEnpoint {
    Coins,
}

impl From<CoingeckoEnpoint> for String {
    fn from(item: CoingeckoEnpoint) -> Self {
        String::from(match item {
            CoingeckoEnpoint::Coins => "/api/v3/coins",
        })
    }
}
