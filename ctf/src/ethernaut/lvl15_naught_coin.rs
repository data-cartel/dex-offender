use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::naught_coin::NaughtCoin;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level15)
    }

    fn name(&self) -> &'static str { "Naught Coin" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_addr: _, offender, offender_addr: _, some_user: _, some_user_addr: _ } = roles;

        println!("Deploying the NaughtCoin contract...");
        let contract =
            NaughtCoin::deploy(deployer, roles.offender_addr).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, deployer_addr: _, offender, offender_addr: _, some_user: _, some_user_addr: _ } = roles;
        let contract = NaughtCoin::new(self.address, deployer);

        println!("Checking that you transfered all tokens...");
        let balance = contract.balanceOf(roles.offender_addr).call().await?;
        let zero = U256::from(0_u8);
        Ok(balance == zero)
    }
}
