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
    RootProvider,
>;

pub(super) async fn get_ws_provider(url: &str) -> Provider {
    let ws_connect = WsConnect::new(url);
    ProviderBuilder::new().connect_ws(ws_connect).await.unwrap()
}

pub(super) fn get_http_provider(input: &str) -> Provider {
    let url = Url::parse(input).unwrap();
    ProviderBuilder::new().connect_http(url)
}
