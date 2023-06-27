use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::Challenge;
use bindings::telephone::Telephone;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Level4 {
    pub contract_address: Address,
}

#[async_trait]
impl Challenge for Level4 {
    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Telephone contract...");
        let contract =
            Telephone::deploy(deployer.to_owned(), ())?.send().await?;

        let owner = contract.owner().await?;
        assert_eq!(owner, deployer.address());

        let challenge = Level4 { contract_address: contract.address() };

        Ok(challenge)
    }

    const DESCRIPTION: &'static str = "Ethernaut
    Level 4: Telephone

    Claim the ownership of the contract to complete this level.
    ";

    async fn check(self, roles: Roles) -> eyre::Result<Level4> {
        let Roles { deployer, .. } = roles;
        let contract = Telephone::new(self.contract_address, deployer.clone());

        println!("Checking that you became the owner...");
        let owner = contract.owner().await?;
        assert_eq!(owner, roles.offender.address());

        Ok(self)
    }
}
