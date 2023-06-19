use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::Level;
use bindings::coin_flip::CoinFlip;

pub struct EthernautLevel3 {
    pub contract_address: Address,
}

#[async_trait]
impl Level for EthernautLevel3 {
    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the CoinFlip contract...");
        let contract =
            CoinFlip::deploy(deployer.to_owned(), ())?.send().await?;

        let consecutive_wins = contract.consecutive_wins().await?;
        assert_eq!(consecutive_wins, 0.into());

        let level = EthernautLevel3 { contract_address: contract.address() };

        Ok(level)
    }

    const DESCRIPTION: &'static str = "Ethernaut
    Level 3: CoinFlip

    This is a coin flipping game where you need to build up your winning streak by
    guessing the outcome of a coin flip. To complete this level you'll need to
    use your psychic abilities to guess the correct outcome 10 times in a row.

    Things that might help:
    - README.md
    ";

    async fn solve(&self, _offender: Actor) -> eyre::Result<()> {
        todo!("Solve me")
    }

    async fn check(self, roles: Roles) -> eyre::Result<EthernautLevel3> {
        let Roles { deployer, .. } = roles;
        let contract = CoinFlip::new(self.contract_address, deployer.clone());

        println!("Checking that you won 10 times in a row...");
        let consecutive_wins = contract.consecutive_wins().await?;
        assert_eq!(consecutive_wins, 10.into());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_level;

    #[tokio::test]
    #[ignore]
    async fn test() -> eyre::Result<()> {
        test_level::<EthernautLevel3>().await?;
        Ok(())
    }
}
