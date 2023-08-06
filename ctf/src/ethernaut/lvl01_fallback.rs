use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::fallback::Fallback;
use crate::{roles::*, to_ether, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level01)
    }

    fn name(&self) -> &'static str { "Fallback" }

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

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Fallback::new(self.address, deployer.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().await?;
        let is_owner = owner == offender.address();

        println!("Checking that you reduced its balance to 0...");
        let contract_balance = deployer.get_balance(self.address, None).await?;
        let balance_reduced = contract_balance == U256::from(0);

        Ok(is_owner && balance_reduced)
    }
}
