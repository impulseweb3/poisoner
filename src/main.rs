use crate::config::get_config;
use crate::providers::{get_http_provider, get_ws_provider};
use alloy::consensus::Transaction;
use alloy::primitives::U256;
use alloy::providers::Provider;
use futures_util::StreamExt;
use std::ops::Mul;

mod config;
mod providers;

#[tokio::main]
async fn main() {
    let config = get_config();

    let target_value = U256::from(config.target.value);
    let target_value = target_value.mul(U256::from(10).pow(U256::from(18)));

    let ws_provider = get_ws_provider(&config.ws_url).await;
    let http_provider = get_http_provider(&config.http_url);

    let mut stream = ws_provider
        .subscribe_full_blocks()
        .full()
        .into_stream()
        .await
        .unwrap();

    while let Some(block) = stream.next().await {
        let block = block.unwrap();

        for transaction in block.into_transactions_iter() {
            if transaction.value() > target_value {
                println!("{:#?}", transaction);
            }
        }
    }
}
