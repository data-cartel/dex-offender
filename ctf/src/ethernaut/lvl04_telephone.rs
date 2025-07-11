use alloy::primitives::Address;
use async_trait::async_trait;

pub use crate::abi::telephone::Telephone;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level04)
    }

    fn name(&self) -> &'static str {
        "Telephone"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_address, .. } = roles;

        println!("Deploying the Telephone contract...");
        let contract = Telephone::deploy(deployer).await?;

        let owner = contract.owner().call().await?._0;
        assert_eq!(owner, *deployer_address);

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender_address, .. } = roles;
        let contract = Telephone::new(self.address, deployer);

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?._0;
        let is_owner = owner == *offender_address;

        Ok(is_owner)
    }
}
