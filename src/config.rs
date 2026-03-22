use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub(super) enum Environment {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "production")]
    Production,
}

#[derive(Debug, Deserialize)]
pub(super) struct Target {
    pub(super) from: bool,
    pub(super) to: bool,
    pub(super) value: usize,
}

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    pub(super) environment: Environment,
    #[serde(rename = "wsUrl")]
    pub(super) ws_url: String,
    #[serde(rename = "httpUrl")]
    pub(super) http_url: String,
    pub(super) target: Target,
    pub(super) prefix: usize,
    pub(super) suffix: usize,
    #[serde(rename = "publicKey")]
    pub(super) public_key: String,
    #[serde(rename = "privateKey")]
    pub(super) private_key: String,
}

pub(super) fn get_config() -> Config {
    let string = fs::read_to_string("config.json").unwrap();
    serde_json::from_str(&string).unwrap()
}
