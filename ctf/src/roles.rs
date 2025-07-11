use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    network::Ethereum,
    signers::{local::PrivateKeySigner, Signer},
    primitives::Address,
};
use std::sync::Arc;

pub type Actor = Arc<RootProvider<Ethereum>>;

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
        let deployer = Arc::new(
            ProviderBuilder::new()
                .with_gas_estimation()
                .wallet(deployer_key)
                .on_http(rpc_url.parse()?)
                .root().clone()
        );

        let some_user_key: PrivateKeySigner =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user_address = some_user_key.address();
        let some_user = Arc::new(
            ProviderBuilder::new()
                .with_gas_estimation()
                .wallet(some_user_key)
                .on_http(rpc_url.parse()?)
                .root().clone()
        );

        let offender_key: PrivateKeySigner =
            "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a".parse()?;
        let offender_address = offender_key.address();
        let offender = Arc::new(
            ProviderBuilder::new()
                .with_gas_estimation()
                .wallet(offender_key)
                .on_http(rpc_url.parse()?)
                .root().clone()
        );

        Ok(Self {
            deployer,
            deployer_address,
            some_user,
            some_user_address,
            offender,
            offender_address,
        })
    }
}
