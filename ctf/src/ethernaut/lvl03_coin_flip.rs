use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::coin_flip::CoinFlip;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level03)
    }

    fn name(&self) -> &'static str { "Coin Flip" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the CoinFlip contract...");
        let contract =
            CoinFlip::deploy(deployer, ()).await?;

        let consecutive_wins = contract.consecutive_wins().call().await?._0;
        assert_eq!(consecutive_wins, U256::from(0));

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = CoinFlip::new(self.address, deployer);

        println!("Checking that you won 10 times in a row...");
        let consecutive_wins = contract.consecutive_wins().call().await?._0;
        let ten_wins = consecutive_wins >= U256::from(10);

        Ok(ten_wins)
    }
}
