use alloy::{
    network::EthereumWallet,
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use std::sync::Arc;

// Type alias for our provider with wallet
pub type Actor = Arc<
    alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::RootProvider<alloy::transports::http::Http<alloy::transports::http::Client>>,
        alloy::transports::http::Http<alloy::transports::http::Client>,
        alloy::network::Ethereum,
    >,
>;

#[derive(Debug, Clone)]
pub struct Roles {
    pub deployer: Actor,
    pub some_user: Actor,
    pub offender: Actor,
}

impl Roles {
    pub fn new(rpc_url: &str) -> eyre::Result<Self> {
        let deployer: PrivateKeySigner =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer = mk_signer(rpc_url, deployer)?;

        let some_user: PrivateKeySigner =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user = mk_signer(rpc_url, some_user)?;

        let offender: PrivateKeySigner =
            "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
        let offender = mk_signer(rpc_url, offender)?;

        Ok(Roles { deployer, some_user, offender })
    }
}

fn mk_signer(
    rpc_url: &str,
    signer: PrivateKeySigner,
) -> eyre::Result<Actor> {
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);
    Ok(Arc::new(provider))
}
