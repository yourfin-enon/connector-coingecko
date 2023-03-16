use crate::rest::config::Config;
use crate::rest::enpoints::CoingeckoEnpoint;
use crate::rest::errors::Error;
use crate::rest::models::Coin;
use error_chain::bail;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::{HashMap};

#[derive(Clone)]
pub struct CoingeckoRestClient {
    api_key: Option<String>,
    host: String,
    inner_client: reqwest::Client,
}

impl CoingeckoRestClient {
    pub fn new(api_key: Option<String>) -> Self {
        CoingeckoRestClient {
            api_key,
            host: Config::default().rest_api_host,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn get_coin(&self, id: impl Into<String>) -> Result<Coin, Error>
    {
        let mut query_params: HashMap<String, String> = HashMap::new();
        query_params.insert("tickers".into(), "false".to_string());
        query_params.insert("market_data".into(), "true".to_string());
        query_params.insert("community_data".into(), "false".to_string());
        query_params.insert("developer_data".into(), "false".to_string());
        query_params.insert("sparkline".into(), "false".to_string());

        let query = self.build_query(query_params);
        self.get(CoingeckoEnpoint::Coins, Some(id.into()), Some(query)).await
    }

    async fn get<T: DeserializeOwned>(
        &self,
        endpoint: CoingeckoEnpoint,
        path: Option<String>,
        query: Option<String>,
    ) -> Result<T, Error> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));

        if let Some(path) = path {
            if !path.is_empty() {
                url.push_str(&format!("/{}", &path));
            }
        }

        if let Some(query) = query {
            if !query.is_empty() {
                url.push_str(format!("?{query}", ).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send().await?;

        self.handler(response).await
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        if let Some(api_key) = self.api_key.as_ref() {
            custom_headers.insert(
                HeaderName::from_static("x-cg-pro-api-key"),
                HeaderValue::from_str(api_key).unwrap(),
            );
        }

        custom_headers
    }

    pub fn build_query(&self, parameters: HashMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{key}={value}&");
            request.push_str(param.as_ref());
        }
        request.pop();
        request
    }

    async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                bail!("Bad request");
            }
            s => {
                bail!(format!("Received response: {s:?}"));
            }
        }
    }
}
