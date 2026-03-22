use crate::config::Config;
use alloy::primitives::Address;
use fern::Dispatch;
use log::LevelFilter;
use std::time::SystemTime;

pub(super) fn setup_logger() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
}

pub(super) fn get_identifier(config: &Config, address: &Address) -> String {
    let address = address.to_string().to_lowercase();

    let prefix = &address[2..2 + config.prefix];
    let suffix = &address[42 - config.suffix..];

    format!("{}{}", prefix, suffix)
}
