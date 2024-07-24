use alloy_primitives::Address;
use listener::listener::EthPubSubClient;
use std::str::FromStr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let ws_url = std::env::var("RETH_WS_URL").expect("RETH_WS_URL not found in .env");
    let owner_address =
        std::env::var("OUR_CONTRACT_ADDRESS").expect("OUR_CONTRACT_ADDRESS not found in .env");
    let client = EthPubSubClient::new_ws(ws_url, Address::from_str(&owner_address)?);
    client.await;

    Ok(())
}
