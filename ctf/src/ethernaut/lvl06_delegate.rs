use async_trait::async_trait;
use ethers::prelude::*;

use crate::{roles::*, Level};
pub use bindings::{delegate::Delegate, delegation::Delegation};

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
            Delegate::deploy(deployer.to_owned(), deployer.address())?
                .send()
                .await?;

        println!("Deploying the Delegation contract...");
        let delegation =
            Delegation::deploy(deployer.to_owned(), delegate.address())?
                .send()
                .await?;

        let owner = delegate.owner().await?;
        assert_eq!(owner, deployer.address());

        let target = Target { delegation_address: delegation.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let delegation =
            Delegation::new(self.delegation_address, deployer.clone());

        println!("Checking that you became the owner...");
        let owner = delegation.owner().await?;
        let is_owner = owner == roles.offender.address();

        Ok(is_owner)
    }
}
