use crate::{roles::*, to_ether, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::reentrance::Reentrance;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level10)
    }

    fn name(&self) -> &'static str { "Re-entrancy" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user } = roles;

        println!("Deploying the Reentrance contract...");
        let contract =
            Reentrance::deploy(deployer.to_owned(), ())?.send().await?;

        deployer
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .value(to_ether(1)),
                None,
            )
            .await?
            .await?;
        contract
            .donate(some_user.address())
            .value(to_ether(20))
            .send()
            .await?
            .await?;

        let contract =
            Reentrance::new(contract.address(), some_user.to_owned());
        contract
            .donate(deployer.address())
            .value(to_ether(100))
            .send()
            .await?
            .await?;

        let target = Target { address: contract.address() };
        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Reentrance::new(self.address, deployer.clone());

        println!("Checking the contract balance...");
        let balance = deployer.get_balance(contract.address(), None).await?;

        Ok(balance == 0.into())
    }
}
