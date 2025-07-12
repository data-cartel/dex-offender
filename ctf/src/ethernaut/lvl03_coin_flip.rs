use alloy::primitives::Address;
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

    fn name(&self) -> &'static str {
        "CoinFlip"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the CoinFlip contract...");
        let contract = CoinFlip::deploy(deployer).await?;

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = CoinFlip::new(self.address, deployer);

        println!("Checking that you won 10 coin flips in a row...");
        let consecutive_wins = contract.consecutiveWins().call().await?._0;
        let solved = consecutive_wins >= 10;

        Ok(solved)
    }
}
