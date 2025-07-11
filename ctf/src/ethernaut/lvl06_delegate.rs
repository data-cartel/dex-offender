use alloy::primitives::Address;
use async_trait::async_trait;

pub use crate::abi::{delegate::Delegate, delegation::Delegation};
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub delegate_address: Address,
    pub delegation_address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level06)
    }

    fn name(&self) -> &'static str {
        "Delegate"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_address, .. } = roles;

        println!("Deploying the Delegate contract...");
        let delegate_contract =
            Delegate::deploy(deployer, *deployer_address).await?;

        println!("Deploying the Delegation contract...");
        let delegation_contract =
            Delegation::deploy(deployer, delegate_contract.address()).await?;

        let owner = delegation_contract.owner().call().await?._0;
        assert_eq!(owner, *deployer_address);

        let target = Target {
            delegate_address: delegate_contract.address(),
            delegation_address: delegation_contract.address(),
        };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender_address, .. } = roles;
        let delegation_contract =
            Delegation::new(self.delegation_address, deployer);

        println!(
            "Checking that you became the owner of the delegation contract..."
        );
        let owner = delegation_contract.owner().call().await?._0;
        let is_owner = owner == *offender_address;

        Ok(is_owner)
    }
}
