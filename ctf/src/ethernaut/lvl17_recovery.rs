use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::recovery::Recovery;
use crate::{abi::recovery_solution::RecoverySolution, roles::*, Level};

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
        let Roles { deployer, deployer_addr: _, offender: _, offender_addr: _, some_user: _, some_user_addr: _ } = roles;

        println!("Deploying the Recovery contract...");
        let contract =
            Recovery::deploy(deployer).await?;

        let pending = contract
            .generateToken(String::from("InitialToken"), U256::from(100000))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let solution_contract =
            RecoverySolution::deploy(deployer).await?;
        let token_address =
            solution_contract.solution(*contract.address()).call().await?;
        let tx = TransactionRequest::default().to(token_address).value(U256::from(100000));
        let pending = deployer.send_transaction(tx).await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, deployer_addr: _, offender: _, offender_addr: _, some_user: _, some_user_addr: _ } = roles;
        let contract = Recovery::new(self.address, deployer);

        let solution_contract =
            RecoverySolution::deploy(deployer).await?;

        let token_address =
            solution_contract.solution(*contract.address()).call().await?;

        println!(
            "Checking that you found the token and took all the ether from \
             it..."
        );

        Ok(deployer.get_balance(token_address).await? == U256::from(0))
    }
}
