use crate::config::Config;
use crate::providers::Provider;
use crate::utils::get_identifier;
use alloy::primitives::Address;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use std::sync::Arc;

pub(crate) async fn poison(
    config: Arc<Config>,
    db: Arc<DBWithThreadMode<SingleThreaded>>,
    http_provider: Arc<Provider>,
    from: Address,
    to: Address,
) {
    let identifier = get_identifier(&config, &from);
}
