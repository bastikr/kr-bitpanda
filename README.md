# KR-Bitpanda

An unofficial, simple async rust client library for the classic (non-pro) [bitpanda](https://www.bitpanda.com) REST API as mostly described by https://developers.bitpanda.com/platform/#bitpanda-public-api. 


## Example

Add `kr-bitpanda` and an async runtime, e.g. [tokio](https://github.com/tokio-rs/tokio) to your crate's dependencies. 

```toml
# Cargo.toml

# ...

[dependencies]
tokio = { version = "1", features = ["rt", "macros"] }
kr-bitpanda = { path = "../kr-bitpanda" }
```

The following `main.rs` can be used as minimal example to get started

```rust
// main.rs
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let ticker = kr_bitpanda::api::public::get_ticker().await?;
    println!("{:#?}", ticker);
    Ok(())
}
```


## Design

Rust structs corresponding to the JSON objects returned by the REST API are implemented and [serde](https://github.com/serde-rs/serde)/[serde_json](https://github.com/serde-rs/json) is used for deserialization. Helper functions are provided for accessing the REST endpoints, separated into publicly available endpoints, which can be accessed without a key, and private endpoints for which an API key is required:

* Public endpoints

    * `kr_bitpanda::api::private::get_ticker()`

* Private endpoints

    * `kr_bitpanda::api::public::get_crypto_wallets(&key)`
    * `kr_bitpanda::api::public::get_fiat_wallets(&key)`

The http requests are performed using the async API of the [reqwest](https://github.com/seanmonstar/reqwest) library, currently without the possibility to reuse the reqwest client.

For more convenient error handling the `anyhow::Result` type is currently used.
