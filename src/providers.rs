use alloy::network::AnyNetwork;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider, WsConnect};
use alloy::transports::http::reqwest::Url;

type Provider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider<AnyNetwork>,
    AnyNetwork,
>;

pub(super) async fn get_ws_provider(url: &str) -> Provider {
    ProviderBuilder::new()
        .network::<AnyNetwork>()
        .connect_ws(WsConnect::new(url))
        .await
        .unwrap()
}

pub(super) fn get_http_provider(input: &str) -> Provider {
    ProviderBuilder::new()
        .network::<AnyNetwork>()
        .connect_http(Url::parse(input).unwrap())
}
