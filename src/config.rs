use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub(super) struct Target {
    pub(super) from: bool,
    pub(super) to: bool,
    pub(super) value: u8,
}

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    #[serde(rename = "wsUrl")]
    pub(super) ws_url: String,
    #[serde(rename = "httpUrl")]
    pub(super) http_url: String,
    pub(super) target: Target,
    #[serde(rename = "privateKey")]
    pub(super) private_key: String,
}

pub(super) fn get_config() -> Config {
    let string = fs::read_to_string("config.json").unwrap();
    serde_json::from_str(&string).unwrap()
}
