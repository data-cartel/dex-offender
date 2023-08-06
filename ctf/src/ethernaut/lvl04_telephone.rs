use async_trait::async_trait;
use ethers::prelude::*;

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

    fn name(&self) -> &'static str { "Telephone" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Telephone contract...");
        let contract =
            Telephone::deploy(deployer.to_owned(), ())?.send().await?;

        let owner = contract.owner().await?;
        assert_eq!(owner, deployer.address());

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Telephone::new(self.address, deployer.clone());

        println!("Checking that you became the owner...");
        let owner = contract.owner().await?;
        let is_owner = owner == roles.offender.address();

        Ok(is_owner)
    }
}
