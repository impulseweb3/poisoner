use crate::config::Config;
use alloy::consensus::Transaction;
use alloy::network::{AnyRpcTransaction, TransactionResponse};
use alloy::primitives::utils::format_ether;
use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Message {
    chat_id: i64,
    text: String,
}

pub(super) async fn send_notification(config: &Config, transaction: &AnyRpcTransaction) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config.telegram.token,
    );

    let text = [
        "**New Transaction**",
        "",
        "From Address",
        &format!("{}", transaction.from().to_string().to_lowercase()),
        "",
        "To Address",
        &format!("{}", transaction.to().unwrap().to_string().to_lowercase()),
        "",
        "Transfer Value",
        &format!("{}", format_ether(transaction.value())),
        "",
        "Transaction Hash",
        &format!("{}", transaction.tx_hash()),
    ];

    let message = Message {
        chat_id: config.telegram.chat_id,
        text: text.join("\n"),
    };

    let response = Client::new().get(url).json(&message).send().await.unwrap();
}
