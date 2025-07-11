use alloy::primitives::{Address, U256};
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

    fn name(&self) -> &'static str {
        "Fallout"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_address, .. } = roles;

        println!("Deploying the Fallout contract...");
        let contract = Fallout::deploy(deployer, ()).await?;

        let receipt = contract.Fal1out().send().await?.get_receipt().await?;
        println!("Called Fal1out: {:?}", receipt.transaction_hash);

        let owner = contract.owner().call().await?._0;
        assert_eq!(owner, *deployer_address);

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender_address, .. } = roles;
        let contract = Fallout::new(self.address, deployer);

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?._0;
        let is_owner = owner == *offender_address;

        Ok(is_owner)
    }
}
