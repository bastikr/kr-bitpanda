use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BestFeeCollection {
    pub attributes: BestFeeCollectionAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BestFeeCollectionAttributes {
    pub best_current_price_eur: Decimal,
    pub best_used_price_eur: Decimal,
    pub bfc_deduction: Decimal,
    pub bfc_market_value_eur: Decimal,
    // TODO: Implement wallet_transaction
    // pub wallet_transaction: ...,
}
