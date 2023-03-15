#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_host: "https://api.coingecko.com".into(),
        }
    }
}