use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::Challenge;
use bindings::coin_flip::CoinFlip;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Level3 {
    pub contract_address: Address,
}

#[async_trait]
impl Challenge for Level3 {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level3)
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the CoinFlip contract...");
        let contract =
            CoinFlip::deploy(deployer.to_owned(), ())?.send().await?;

        let consecutive_wins = contract.consecutive_wins().await?;
        assert_eq!(consecutive_wins, 0.into());

        let challenge = Level3 { contract_address: contract.address() };

        Ok(challenge)
    }

    const DESCRIPTION: &'static str = "Ethernaut
    Level 3: CoinFlip

    This is a coin flipping game where you need to build up your winning streak by
    guessing the outcome of a coin flip. To complete this challenge you'll need to
    use your psychic abilities to guess the correct outcome 10 times in a row.

    Things that might help:
    - README.md
    ";

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = CoinFlip::new(self.contract_address, deployer.clone());

        println!("Checking that you won 10 times in a row...");
        let consecutive_wins = contract.consecutive_wins().await?;
        let ten_wins = consecutive_wins == 10.into();

        Ok(ten_wins)
    }
}
