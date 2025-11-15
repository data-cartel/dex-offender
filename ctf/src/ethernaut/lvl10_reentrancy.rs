use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::reentrance::Reentrance;
use crate::{roles::*, to_ether, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level10)
    }

    fn name(&self) -> &'static str { "Re-entrancy" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user } = roles;

        println!("Deploying the Reentrance contract...");
        let contract =
            Reentrance::deploy(deployer, ()).await?;

        let tx = TransactionRequest::default()
            .to(*contract.address())
            .value(to_ether(1));
        let pending = deployer.send_transaction(tx).await?;
        let _receipt = pending.get_receipt().await?;

        let pending = contract
            .donate(some_user.default_signer_address())
            .value(to_ether(20))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let contract =
            Reentrance::new(*contract.address(), some_user);
        let pending = contract
            .donate(deployer.default_signer_address())
            .value(to_ether(100))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };
        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Reentrance::new(self.address, deployer);

        println!("Checking the contract balance...");
        let balance = deployer.get_balance(*contract.address()).await?;

        Ok(balance == U256::from(0))
    }
}
