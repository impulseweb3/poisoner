use crate::config::Config;
use crate::utils::get_identifier;
use alloy::consensus::Transaction;
use alloy::network::{
    AnyNetwork, AnyRpcTransaction, EthereumWallet, NetworkWallet, TransactionResponse,
};
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use log::info;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use std::str::FromStr;
use std::sync::Arc;

pub(super) fn tracker(
    config: &Config,
    db: &Arc<DBWithThreadMode<SingleThreaded>>,
    transaction: &AnyRpcTransaction,
) {
    let public_key = Address::from_str(&config.public_key).unwrap();

    if !transaction.from() == public_key && transaction.to().is_some() {
        let to = transaction.to().unwrap();
        let identifier = get_identifier(&config, &to);

        let bytes = db.get(identifier).unwrap().unwrap();
        let string = String::from_utf8(bytes).unwrap();

        let signer = PrivateKeySigner::from_str(&string).unwrap();
        let wallet = EthereumWallet::from(signer);

        if NetworkWallet::<AnyNetwork>::default_signer_address(&wallet) == to {
            let address = to.to_string().to_lowercase();
            let value = transaction.value();
            info!("received ethereum | address {} value {}", address, value);
        }
    }
}
