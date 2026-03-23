use crate::config::Config;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Message {
    chat_id: i64,
    message_thread_id: u8,
    text: String,
}

pub(super) async fn send_notification(config: &Config, message_thread_id: u8, text: String) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config.telegram.incoming
    );

    let message = Message {
        chat_id: config.telegram.chat,
        message_thread_id,
        text,
    };

    let response = reqwest::Client::new()
        .get(url)
        .json(&message)
        .send()
        .await
        .unwrap();
}

pub(super) async fn send_incoming_transaction(config: &Config) {
    send_notification(config, config.telegram.incoming, "".to_string()).await;
}

pub(super) async fn send_outcoming_transaction(config: &Config) {
    send_notification(config, config.telegram.outcoming, "".to_string()).await;
}
