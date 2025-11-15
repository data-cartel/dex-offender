use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::fallout::Fallout;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level02)
    }

    fn name(&self) -> &'static str { "Fallout" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Fallout contract...");
        let contract = Fallout::deploy(deployer, ()).await?;

        let pending = contract.fal_1out().send().await?;
        let _receipt = pending.get_receipt().await?;

        let owner = contract.owner().call().await?._0;
        assert_eq!(owner, deployer.default_signer_address());

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Fallout::new(self.address, deployer);

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?._0;
        let is_owner = owner == offender.default_signer_address();

        Ok(is_owner)
    }
}
