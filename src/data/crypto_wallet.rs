use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CryptocoinSymbol {
    BTC,
    LTC,
    ETH,
    LSK,
    DASH,
    XRP,
    BCH,
    PAN,
    KMD,
    MIOTA,
    EOS,
    OMG,
    REP,
    ZRX,
    ZEC,
    XEM,
    XLM,
    XTZ,
    ADA,
    NEO,
    ETC,
    LINK,
    WAVES,
    USDT,
    USDC,
    TRX,
    ATOM,
    BEST,
    BAT,
    CHZ,
    DOGE,
    ONT,
    QTUM,
    VET,
    DOT,
    FIL,
    YFI,
    MKR,
    COMP,
    SNX,
    UNI,
    AAVE,
    KNC,
    BAND,
    REN,
    UMA,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CryptoWallets {
    pub data: Vec<CryptoWallet>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CryptoWallet {
    pub id: String,
    pub attributes: CryptoWalletAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CryptoWalletAttributes {
    pub cryptocoin_id: String,
    pub cryptocoin_symbol: CryptocoinSymbol,
    pub balance: Decimal,
    pub is_default: bool,
    pub name: String,
    pub pending_transactions_count: u64,
    pub deleted: bool,
    pub is_index: bool,
}

#[test]
fn test_crypto_wallets_deserialization() -> anyhow::Result<()> {
    let data = include_str!("../../test/assets/crypto_wallets.json");
    let _v: CryptoWallets = serde_json::from_str(data)?;
    Ok(())
}
