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
        let contract = Token::deploy(deployer.as_ref(), U256::from(21_000_000)).await?;

        let pending = contract.transfer(offender.default_signer_address(), U256::from(20)).send().await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Token::new(self.address, deployer.as_ref());

        println!("Checking that got more tokens...");
        let balance = contract.balance_of(offender.default_signer_address()).call().await?._0;

        Ok(balance > U256::from(20))
    }
}
