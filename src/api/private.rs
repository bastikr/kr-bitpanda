use anyhow::{ensure, Result};

use crate::data::{CryptoWallets, FiatWalletTransactions, FiatWallets, Trades};

pub async fn get_crypto_wallets(key: &str) -> Result<CryptoWallets> {
    let wallets_url = "https://api.bitpanda.com/v1/wallets";
    let client = reqwest::Client::new();
    let response = client
        .get(wallets_url)
        .header("X-API-KEY", key)
        .send()
        .await?;
    let status = response.status();
    ensure!(
        status == reqwest::StatusCode::OK,
        "response status not ok: {}",
        status
    );
    let wallets: CryptoWallets = response.json().await?;
    Ok(wallets)
}

pub async fn get_fiat_wallets(key: &str) -> Result<FiatWallets> {
    let wallets_url = "https://api.bitpanda.com/v1/fiatwallets";
    let client = reqwest::Client::new();
    let response = client
        .get(wallets_url)
        .header("X-API-KEY", key)
        .send()
        .await?;
    let status = response.status();
    ensure!(
        status == reqwest::StatusCode::OK,
        "response status not ok: {}",
        status
    );
    let wallets: FiatWallets = response.json().await?;
    Ok(wallets)
}

// TODO: Handle multi-page trades
pub async fn get_trades(key: &str) -> Result<Trades> {
    let url = "https://api.bitpanda.com/v1/trades";
    let client = reqwest::Client::new();
    let response = client.get(url).header("X-API-KEY", key).send().await?;
    let status = response.status();
    ensure!(
        status == reqwest::StatusCode::OK,
        "response status not ok: {}",
        status
    );
    let trades: Trades = response.json().await?;
    Ok(trades)
}

// TODO: Handle multi-page trades
pub async fn get_fiat_wallet_transactions(key: &str) -> Result<FiatWalletTransactions> {
    let url = "https://api.bitpanda.com/v1/fiatwallets/transactions";
    let client = reqwest::Client::new();
    let response = client.get(url).header("X-API-KEY", key).send().await?;
    let status = response.status();
    ensure!(
        status == reqwest::StatusCode::OK,
        "response status not ok: {}",
        status
    );
    let transactions: FiatWalletTransactions = response.json().await?;
    Ok(transactions)
}
