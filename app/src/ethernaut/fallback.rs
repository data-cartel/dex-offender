use async_trait::async_trait;
use ethers::prelude::*;

use crate::dex_offenders::*;
use crate::Level;
use bindings::fallback::Fallback;

/*
Level 1: Fallback

Look carefully at the contract's code below.

You will beat this level if
1. you claim ownership of the contract
2. you reduce its balance to 0

Things that might help:
- How to send ether when interacting with an ABI
- How to send ether outside of the ABI
- Converting to and from wei/ether units (see help() command)
- Fallback methods
*/

pub struct EthernautLevel1 {
    pub contract: Fallback<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[async_trait]
impl Level for EthernautLevel1 {
    async fn set_up(offenders: &DexOffenders) -> eyre::Result<EthernautLevel1> {
        let DexOffenders { deployer, player, some_user: _ } = offenders;

        let contract = Fallback::deploy(deployer.to_owned(), ())?;
        let contract = contract.send().await?;

        let balance = contract.contributions(deployer.address()).await?;
        assert_eq!(balance, U256::from(1000));

        let balance = contract.contributions(player.address()).await?;
        assert_eq!(balance, U256::from(0));

        let level = EthernautLevel1 { contract };

        Ok(level)
    }

    #[allow(unused_variables)]
    async fn solve(&self, player: &DexOffender) -> eyre::Result<()> {
        unimplemented!("Solve me!")
    }

    async fn validate(self, offenders: &DexOffenders) -> eyre::Result<EthernautLevel1> {
        let EthernautLevel1 { contract } = self;
        let DexOffenders { deployer, player, some_user: _ } = offenders;

        let deployer_balance = contract.contributions(deployer.address()).await?;
        assert_eq!(deployer_balance, U256::from(0));

        let player_balance = contract.contributions(player.address()).await?;
        assert_eq!(player_balance, U256::from(1000));

        let owner = contract.owner().await?;
        assert_eq!(owner, player.address());

        Ok(EthernautLevel1 { contract })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_level;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        test_level::<EthernautLevel1>().await?;
        Ok(())
    }
}
