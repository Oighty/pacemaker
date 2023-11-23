// common.rs - utility functions used across modules

use anyhow::Result;
use ethers::utils::hex;
use reqwest::Client;
use std::time::SystemTime;

// Function to return system time in seconds as a u64
pub fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

// Format ethers types as 0x-prefixed hex strings

pub fn bytes_to_string(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

pub async fn get_token_price(web_client: &Client, token: &str) -> Result<f64> {
    let url = format!("https://coins.llama.fi/prices/current/coingecko:{}", token);
    let payload = web_client
        .get(&url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let price = payload["coins"][format!("coingecko:{}", token)]["price"]
        .as_f64()
        .unwrap();
    Ok(price)
}
