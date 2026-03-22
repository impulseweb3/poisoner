use crate::config::Config;
use alloy::network::AnyRpcTransaction;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use std::sync::Arc;

pub(super) fn tracker(
    config: &Config,
    db: &Arc<DBWithThreadMode<SingleThreaded>>,
    transaction: &AnyRpcTransaction,
) {
}
