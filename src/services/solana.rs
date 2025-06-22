
use reqwest::Client;
use serde_json::json;

pub async fn get_solana_balance(pubkey: &str) -> Result<f64, reqwest::Error> {
    let client = Client::new();
    let payload = json!({

    });


    Ok(lamports / 1_000_000_000)



}