use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::Challenge;
use bindings::delegate::Delegate;
use bindings::delegation::Delegation;

pub struct EthernautLevel6 {
    pub delegation_address: Address,
    delegate_address: Address,
}

#[async_trait]
impl Challenge for EthernautLevel6 {
    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Delegate contract...");
        let delegate =
            Delegate::deploy(deployer.to_owned(), deployer.address())?
                .send()
                .await?;

        println!("Deploying the Delegation contract...");
        let delegation =
            Delegation::deploy(deployer.to_owned(), delegate.address())?
                .send()
                .await?;

        let owner = delegate.owner().await?;
        assert_eq!(owner, deployer.address());

        let challenge = EthernautLevel6 {
            delegation_address: delegation.address(),
            delegate_address: delegate.address(),
        };

        Ok(challenge)
    }

    const DESCRIPTION: &'static str = "Ethernaut
    Level 6: Delegation

    The goal of this level is for you to claim ownership of the instance you are given.

    Things that might help
    - Look into Solidity's documentation on the delegatecall low level function,
        how it works, how it can be used to delegate operations to on-chain libraries,
        and what implications it has on execution scope.
    - Fallback methods
    - Method ids
    ";

    async fn check(self, roles: Roles) -> eyre::Result<EthernautLevel6> {
        let Roles { deployer, .. } = roles;
        let delegation =
            Delegation::new(self.delegation_address, deployer.clone());

        println!("Checking that you became the owner...");
        let owner = delegation.owner().await?;
        assert_eq!(owner, roles.offender.address());

        Ok(self)
    }
}
