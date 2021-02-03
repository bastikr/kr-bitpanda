use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::FiatSymbol;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum TickerSymbol {
    BTC,
    BEST,
    BCI5,
    ETH,
    XAU,
    BCI10,
    BCI25,
    XAG,
    XRP,
    USDT,
    ADA,
    PAN,
    MIOTA,
    LTC,
    LINK,
    XLM,
    DOT,
    YFI,
    DOGE,
    VET,
    BCH,
    USDC,
    TRX,
    CHZ,
    XEM,
    SNX,
    DASH,
    UNI,
    EOS,
    WAVES,
    XTZ,
    NEO,
    AAVE,
    OMG,
    BAND,
    FIL,
    ZEC,
    REN,
    UMA,
    ETC,
    ONT,
    COMP,
    MKR,
    ZRX,
    ATOM,
    QTUM,
    REP,
    KMD,
    KNC,
    BAT,
    XPT,
    LSK,
    XPD,
    OCEAN,
    ANT,
    BTT,
    DGB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker(pub HashMap<TickerSymbol, HashMap<FiatSymbol, Decimal>>);

#[test]
fn test_crypto_wallets_deserialization() -> anyhow::Result<()> {
    let data = include_str!("../../test/assets/ticker.json");
    let _v: Ticker = serde_json::from_str(data)?;
    let data = include_str!("../../test/assets/ticker-2021-02.json");
    let _v: Ticker = serde_json::from_str(data)?;
    Ok(())
}
