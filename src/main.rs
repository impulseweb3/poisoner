use crate::config::get_config;
use crate::providers::{get_http_provider, get_ws_provider};
use alloy::providers::Provider;
use futures_util::StreamExt;

mod config;
mod providers;

#[tokio::main]
async fn main() {
    let config = get_config();

    let ws_provider = get_ws_provider(&config.ws_url).await;
    let http_provider = get_http_provider(&config.http_url);

    let mut stream = ws_provider
        .subscribe_full_blocks()
        .into_stream()
        .await
        .unwrap();

    while let Some(block) = stream.next().await {
        println!("{}", block.unwrap().header.number);
    }
}
