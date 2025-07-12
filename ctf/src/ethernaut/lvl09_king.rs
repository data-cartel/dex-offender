use crate::{roles::*, to_ether, Level};
use alloy::{primitives::Address, rpc::types::TransactionRequest};
use async_trait::async_trait;

pub use crate::abi::king::King;

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

    fn name(&self) -> &'static str {
        "King"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the King contract...");
        let king = King::deploy(deployer, ()).value(to_ether(10)).await?;

        let target = Target { address: king.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;

        println!("Attempting to reclaim the kingdom...");
        let tx_request =
            TransactionRequest::default().to(self.address).value(to_ether(10));

        let result = deployer.send_transaction(tx_request).await;

        if result.is_err() {
            return Ok(true);
        }

        let pending_tx = result?;
        let result = pending_tx.get_receipt().await;

        Ok(result.is_err())
    }
}
