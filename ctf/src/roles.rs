use alloy::{
    providers::{ProviderBuilder, RootProvider},
    network::Ethereum,
    signers::{local::PrivateKeySigner, wallet::EthereumWallet},
    transports::http::{Http, Client},
    primitives::Address,
};
use std::sync::Arc;

pub type Actor = Arc<RootProvider<Http<Client>>>;

#[derive(Debug, Clone)]
pub struct Roles {
    pub deployer: Actor,
    pub deployer_address: Address,
    pub some_user: Actor, 
    pub some_user_address: Address,
    pub offender: Actor,
    pub offender_address: Address,
}

impl Roles {
    pub async fn new(rpc_url: &str) -> eyre::Result<Self> {
        let deployer_key: PrivateKeySigner = 
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer_address = deployer_key.address();
        let deployer = mk_provider_with_signer(rpc_url, deployer_key).await?;

        let some_user_key: PrivateKeySigner =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user_address = some_user_key.address();
        let some_user = mk_provider_with_signer(rpc_url, some_user_key).await?;

        let offender_key: PrivateKeySigner =
            "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
        let offender_address = offender_key.address();
        let offender = mk_provider_with_signer(rpc_url, offender_key).await?;

        Ok(Roles { 
            deployer, 
            deployer_address,
            some_user, 
            some_user_address,
            offender,
            offender_address 
        })
    }
}

async fn mk_provider_with_signer(
    rpc_url: &str,
    signer: PrivateKeySigner,
) -> eyre::Result<Actor> {
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?)
        .await?;
    
    Ok(Arc::new(provider))
}
