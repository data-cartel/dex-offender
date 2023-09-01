use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{
    library_contract::LibraryContract, preservation::Preservation,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level16)
    }

    fn name(&self) -> &'static str { "Preservation" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Preservation contract...");
        let timezone1 =
            LibraryContract::deploy(deployer.to_owned(), ())?.send().await?;
        let timezone2 =
            LibraryContract::deploy(deployer.to_owned(), ())?.send().await?;
        let contract = Preservation::deploy(
            deployer.to_owned(),
            (timezone1.address(), timezone2.address()),
        )?
        .send()
        .await?;


        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Preservation::new(self.address, deployer.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().await?;
        Ok(owner == offender.address())
    }
}
