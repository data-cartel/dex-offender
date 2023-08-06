use async_trait::async_trait;
use ethers::prelude::*;

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
            CoinFlip::deploy(deployer.to_owned(), ())?.send().await?;

        let consecutive_wins = contract.consecutive_wins().await?;
        assert_eq!(consecutive_wins, 0.into());

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = CoinFlip::new(self.address, deployer.clone());

        println!("Checking that you won 10 times in a row...");
        let consecutive_wins = contract.consecutive_wins().await?;
        let ten_wins = consecutive_wins >= 10.into();

        Ok(ten_wins)
    }
}
