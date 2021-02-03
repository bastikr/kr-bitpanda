use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWalletTransactions {
    pub data: Vec<FiatWalletTransaction>,
    pub meta: Meta,
    pub links: Links,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meta {
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Links {
    pub next: Option<String>,
    pub last: Option<String>,
    #[serde(rename = "self")]
    pub _self: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWalletTransaction {
    pub id: String,
    pub attributes: FiatWalletTransactionAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FiatWalletTransactionAttributes {
    pub fiat_wallet_id: String,
    pub user_id: String,
    pub fiat_id: String,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub trade: Option<super::Trade>,
    pub amount: Decimal,
    pub fee: Decimal,
    pub to_eur_rate: Decimal,
    pub time: super::TradeTime,
    pub in_or_out: TransactionDirection,
    pub status: TradeStatus,
    pub confirmation_by: String,
    pub confirmed: bool,
    pub requires_2fa_approval: bool,
    pub is_savings: bool,
    pub recipient: Option<String>,
    pub from: Option<String>,
    pub last_changed: super::TradeTime,
    pub tags: Vec<String>,
    pub public_status: TradeStatus,
    pub bank_account_details: Option<BankAccountDetails>,
    pub is_index: bool,
    pub is_card: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Transfer,
    Buy,
    Sell,
    Deposit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeStatus {
    Finished,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BankAccountDetails {
    pub attributes: BankAccountAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BankAccountAttributes {
    pub country_id: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: String,
    pub holder: String,
}

#[test]
fn test_fiat_wallet_transactions_deserialization() -> anyhow::Result<()> {
    use anyhow::ensure;
    let data = include_str!("../../test/assets/fiat_wallet_transactions.json");
    let transactions: FiatWalletTransactions = serde_json::from_str(data)?;
    for t in transactions.data {
        match t.attributes.transaction_type {
            TransactionType::Transfer => {
                ensure!(t.attributes.recipient.is_some());
                ensure!(t.attributes.from.is_some());
            }
            TransactionType::Buy => {
                ensure!(t.attributes.trade.is_some());
            }
            TransactionType::Sell => {
                ensure!(t.attributes.trade.is_some());
            }
            TransactionType::Deposit => {
                ensure!(t.attributes.bank_account_details.is_some())
            }
        }
    }
    Ok(())
}
