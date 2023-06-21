use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::{to_ether, Challenge};
use bindings::fallback::Fallback;

pub struct EthernautLevel1 {
    pub contract_address: Address,
}

#[async_trait]
impl Challenge for EthernautLevel1 {
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

        let challenge =
            EthernautLevel1 { contract_address: contract.address() };

        Ok(challenge)
    }

    const DESCRIPTION: &'static str = "Ethernaut
    Level 1: Fallback

    Look carefully at the contract's code below.

    You will beat this challenge if
    1. you claim ownership of the contract
    2. you reduce its balance to 0

    Things that might help:
    - How to send ether when interacting with an ABI
    - How to send ether outside of the ABI
    - Converting to and from wei/ether units (see help() command)
    - Fallback methods
    ";

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
