use connector_coingecko::rest::rest_client::CoingeckoRestClient;

#[tokio::main]
async fn main() {
    let client = CoingeckoRestClient::new(None);
    let coin = client.get_coin("bitcoin").await;

    if let Err(err) = coin {
        println!("error: {err:?}");
    } else {
        println!("coin: {:?}", coin.unwrap());
    }
}