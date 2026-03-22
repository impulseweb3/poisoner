use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider, WsConnect};
use alloy::transports::http::reqwest::Url;

pub(super) type WsProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider<AnyNetwork>,
    AnyNetwork,
>;

pub type HttpProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<AnyNetwork>,
    AnyNetwork,
>;

pub(super) async fn get_ws_provider(url: &str) -> WsProvider {
    ProviderBuilder::new()
        .network::<AnyNetwork>()
        .connect_ws(WsConnect::new(url))
        .await
        .unwrap()
}

pub(super) fn get_http_provider(wallet: EthereumWallet, input: &str) -> HttpProvider {
    ProviderBuilder::new()
        .wallet(wallet)
        .network::<AnyNetwork>()
        .connect_http(Url::parse(input).unwrap())
}
