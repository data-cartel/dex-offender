use crate::{roles::*, to_ether, Level};
use async_trait::async_trait;
use ethers::prelude::*;

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

    fn name(&self) -> &'static str { "King" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the King contract...");
        let king = King::deploy(deployer.to_owned(), ())?
            .value(to_ether(10))
            .send()
            .await?;

        let target = Target { address: king.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = King::new(self.address, deployer.clone());

        println!("Attempting to reclaim the kingdom...");
        let result = deployer
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .value(to_ether(10)),
                None,
            )
            .await;

        if result.is_err() {
            return Ok(true);
        }

        let result = result?.await;

        Ok(result.is_err())
    }
}
