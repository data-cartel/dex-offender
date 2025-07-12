use crate::{roles::*, Level};
use alloy::primitives::{Address, U256};
use async_trait::async_trait;

pub use crate::abi::force::Force;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level07)
    }

    fn name(&self) -> &'static str {
        "Force"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Force contract...");
        let force = Force::deploy(deployer, ()).await?;

        let target = Target { address: force.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;

        println!("Checking the contract balance...");
        let balance = deployer.get_balance(self.address).await?;

        Ok(balance > U256::from(0))
    }
}
