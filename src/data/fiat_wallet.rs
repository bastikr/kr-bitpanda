use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum FiatSymbol {
    EUR,
    USD,
    CHF,
    GBP,
    TRY,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWallets {
    pub data: Vec<FiatWallet>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWallet {
    pub id: String,
    pub attributes: FiatWalletAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWalletAttributes {
    pub fiat_id: String,
    pub fiat_symbol: FiatSymbol,
    pub balance: Decimal,
    pub name: String,
    pub pending_transactions_count: u64,
}

#[test]
fn test_fiat_wallets_deserialization() -> anyhow::Result<()> {
    let data = include_str!("../../test/assets/fiat_wallets.json");
    let _v: FiatWallets = serde_json::from_str(data)?;
    Ok(())
}
