use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::FiatSymbol;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum TickerSymbol {
    AAVE,
    AVAX,
    ADA,
    ANT,
    ATOM,
    BAND,
    BAT,
    BCH,
    BCI10,
    BCI25,
    BCI5,
    BEST,
    BTC,
    BTT,
    CHZ,
    COMP,
    DASH,
    DGB,
    DOGE,
    DOT,
    EOS,
    ETC,
    ETH,
    FIL,
    KMD,
    KNC,
    LINK,
    LSK,
    LTC,
    MIOTA,
    MKR,
    NEO,
    OCEAN,
    OMG,
    ONT,
    PAN,
    QTUM,
    REN,
    REP,
    SNX,
    TRX,
    UMA,
    UNI,
    USDC,
    USDT,
    VET,
    WAVES,
    XAG,
    XAU,
    XEM,
    XLM,
    XPD,
    XPT,
    XRP,
    XTZ,
    YFI,
    ZEC,
    ZRX,
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
