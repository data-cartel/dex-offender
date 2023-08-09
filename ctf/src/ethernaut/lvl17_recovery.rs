use crate::{abi::recovery_solution::RecoverySolution, roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::recovery::Recovery;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level17)
    }

    fn name(&self) -> &'static str { "Recovery" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Recovery contract...");
        let contract =
            Recovery::deploy(deployer.to_owned(), ())?.send().await?;

        contract
            .generate_token(String::from("InitialToken"), U256::from(100000))
            .send()
            .await?;

        let solution_contract =
            RecoverySolution::deploy(deployer.to_owned(), ())?.send().await?;
        let token_address =
            solution_contract.solution(contract.address()).call().await?;
        deployer
            .send_transaction(
                TransactionRequest::new().to(token_address).value(100000),
                None,
            )
            .await?
            .await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = Recovery::new(self.address, deployer.clone());

        let solution_contract =
            RecoverySolution::deploy(deployer.to_owned(), ())?.send().await?;

        let token_address =
            solution_contract.solution(contract.address()).call().await?;

        println!(
            "Checking that you found the token and took all the ether from \
             it..."
        );

        Ok(deployer.get_balance(token_address, None).await? == 0.into())
    }
}
