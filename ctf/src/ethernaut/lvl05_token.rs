use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::token::Token;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level05)
    }

    fn name(&self) -> &'static str { "Token" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, .. } = roles;

        println!("Deploying the Token contract...");
        let contract = Token::deploy(deployer.clone(), U256::from(21_000_000))?
            .send()
            .await?;

        contract.transfer(offender.address(), 20.into()).send().await?;

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Token::new(self.address, deployer.clone());

        println!("Checking that got more tokens...");
        let balance = contract.balance_of(offender.address()).call().await?;

        Ok(balance > 20.into())
    }
}
