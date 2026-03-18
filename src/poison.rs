use crate::providers::Provider;
use alloy::primitives::Address;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use std::sync::Arc;

pub(crate) async fn poison(
    db: Arc<DBWithThreadMode<SingleThreaded>>,
    http_provider: Arc<Provider>,
    address: Address,
) {
}
