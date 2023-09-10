use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::switch::Switch;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level29)
    }

    fn name(&self) -> &'static str { "Switch" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Switch contract...");
        let contract = Switch::deploy(deployer.to_owned(), ())?.send().await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = Switch::new(self.address, deployer.clone());

        println!("Checking that the switch is on...");

        Ok(contract.switch_on().await?)
    }
}
