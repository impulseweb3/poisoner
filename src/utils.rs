use crate::config::Config;
use alloy::primitives::Address;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;
use std::time::SystemTime;

pub(super) fn setup_logger() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue);

    let stdout_dispatch = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    let file_dispatch = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("output.log").unwrap());

    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(stdout_dispatch)
        .chain(file_dispatch)
        .apply()
        .unwrap();
}

pub(super) fn get_identifier(config: &Config, address: &Address) -> String {
    let address = address.to_string().to_lowercase();

    let prefix = &address[2..2 + config.prefix];
    let suffix = &address[42 - config.suffix..];

    format!("{}{}", prefix, suffix)
}
