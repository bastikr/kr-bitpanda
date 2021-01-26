use anyhow::{ensure, Result};

use crate::data::Ticker;

pub async fn get_ticker() -> Result<Ticker> {
    let ticker_url = "https://api.bitpanda.com/v1/ticker";
    let client = reqwest::Client::new();
    let response = client.get(ticker_url).send().await?;
    let status = response.status();
    ensure!(
        status == reqwest::StatusCode::OK,
        "response status not ok: {}",
        status
    );
    let ticker: Ticker = response.json().await?;
    Ok(ticker)
}
