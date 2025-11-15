use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::king::King;
use crate::{roles::*, to_ether, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level09)
    }

    fn name(&self) -> &'static str { "King" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the King contract...");
        let king = King::deploy_builder(deployer).value(to_ether(10)).deploy().await?;

        let target = Target { address: king };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = King::new(self.address, deployer);

        println!("Attempting to reclaim the kingdom...");
        let tx = TransactionRequest::default()
            .to(*contract.address())
            .value(to_ether(10));
        let result = deployer.send_transaction(tx).await;

        if result.is_err() {
            return Ok(true);
        }

        let result = result?.get_receipt().await;

        Ok(result.is_err())
    }
}
