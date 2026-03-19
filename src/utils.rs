use crate::config::Config;
use alloy::primitives::Address;

pub(super) fn get_identifier(config: &Config, address: &Address) -> String {
    let address = address.to_string().to_lowercase();

    let prefix = &address[2..2 + config.prefix];
    let suffix = &address[42 - config.suffix..];

    format!("{}{}", prefix, suffix)
}
