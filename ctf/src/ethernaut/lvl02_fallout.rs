use async_trait::async_trait;
use ethers::prelude::*;

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
        let contract = Fallout::deploy(deployer.to_owned(), ())?.send().await?;

        contract.fal_1out().send().await?;

        let owner = contract.owner().await?;
        assert_eq!(owner, deployer.address());

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Fallout::new(self.address, deployer.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().await?;
        let is_owner = owner == offender.address();

        Ok(is_owner)
    }
}
