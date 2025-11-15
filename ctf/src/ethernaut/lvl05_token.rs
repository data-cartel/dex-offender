use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

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
        let contract = Token::deploy(deployer, U256::from(21_000_000)).await?;

        let pending = contract.transfer(roles.offender_addr, U256::from(20)).send().await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, deployer_addr: _, offender, offender_addr: _, some_user: _, some_user_addr: _ } = roles;
        let contract = Token::new(self.address, deployer);

        println!("Checking that got more tokens...");
        let balance = contract.balanceOf(roles.offender_addr).call().await?;

        Ok(balance > U256::from(20))
    }
}
