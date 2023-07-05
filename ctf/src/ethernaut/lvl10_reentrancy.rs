use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use bindings::reentrance::Reentrance;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub contract_address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level10)
    }

    fn name(&self) -> &'static str { "Re-entrancy" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Reentrance contract...");
        let contract =
            Reentrance::deploy(deployer.to_owned(), ())?.send().await?;

        deployer
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .value(U256::from(1)),
                None,
            )
            .await?
            .await?;

        let target = Target { contract_address: contract.address() };
        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Reentrance::new(self.contract_address, deployer.clone());

        println!("Checking the contract balance...");
        let balance = deployer.get_balance(contract.address(), None).await?;

        Ok(balance == 0.into())
    }
}
