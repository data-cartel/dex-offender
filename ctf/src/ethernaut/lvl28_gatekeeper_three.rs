use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{
    gatekeeper_three::GatekeeperThree, simple_trick::SimpleTrick,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level28)
    }

    fn name(&self) -> &'static str { "Gatekeeper Three" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the GatekeeperThree contract...");
        let contract =
            GatekeeperThree::deploy(deployer.to_owned(), ())?.send().await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = GatekeeperThree::new(self.address, deployer.clone());

        println!("Checking the entrant...");

        let entrant = contract.entrant().await?;
        let pass = entrant == roles.offender.address();

        Ok(pass)
    }
}
