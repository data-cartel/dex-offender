use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::{delegate::Delegate, delegation::Delegation};
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub delegation_address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level06)
    }

    fn name(&self) -> &'static str { "Delegation" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Delegate contract...");
        let delegate =
            Delegate::deploy(deployer, roles.deployer_addr).await?;

        println!("Deploying the Delegation contract...");
        let delegation =
            Delegation::deploy(deployer, *delegate.address()).await?;

        let owner = delegate.owner().call().await?;
        assert_eq!(owner, roles.deployer_addr);

        let target = Target { delegation_address: *delegation.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let delegation =
            Delegation::new(self.delegation_address, deployer);

        println!("Checking that you became the owner...");
        let owner = delegation.owner().call().await?;
        let is_owner = owner == roles.offender_addr;

        Ok(is_owner)
    }
}
