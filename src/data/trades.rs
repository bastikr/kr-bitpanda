use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::BestFeeCollection;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trades {
    pub data: Vec<Trade>,
    pub meta: TradeMeta,
    pub links: TradeLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeMeta {
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeLinks {
    pub next: Option<String>,
    pub last: Option<String>,
    #[serde(rename = "self")]
    pub _self: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    pub id: String,
    pub attributes: TradeAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeAttributes {
    pub status: String,
    #[serde(rename = "type")]
    pub trade_type: TradeType,
    // TODO: Map cryptocoin id to enum
    pub cryptocoin_id: String,
    // TODO: Map fiat id to enum
    pub fiat_id: String,
    pub amount_fiat: Decimal,
    pub amount_cryptocoin: Decimal,
    pub fiat_to_eur_rate: Decimal,
    pub wallet_id: String,
    pub fiat_wallet_id: String,
    pub payment_option_id: Option<String>,
    pub time: TradeTime,
    pub price: Decimal,
    pub is_swap: bool,
    pub related_swap_trade: Option<Box<Trade>>,
    pub is_savings: bool,
    pub tags: Vec<String>,
    pub bfc_used: bool,
    pub best_fee_collection: Option<BestFeeCollection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeType {
    Buy,
    Sell,
}

// TODO: Conversion to rust (chrono?) date types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeTime {
    pub date_iso8601: String,
    pub unix: String,
}

#[test]
fn test_fiat_wallets_deserialization() -> anyhow::Result<()> {
    let data = include_str!("../../test/assets/trades.json");
    let trades: Trades = serde_json::from_str(data)?;
    for trade in trades.data {
        if trade.attributes.bfc_used {
            assert!(trade.attributes.best_fee_collection.is_some());
        }
        if trade.attributes.is_swap {
            assert!(trade.attributes.related_swap_trade.is_some());
        }
    }
    Ok(())
}
