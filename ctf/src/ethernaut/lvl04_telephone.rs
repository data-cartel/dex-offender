use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::telephone::Telephone;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level04)
    }

    fn name(&self) -> &'static str { "Telephone" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Telephone contract...");
        let contract =
            Telephone::deploy(deployer).await?;

        let owner = contract.owner().call().await?;
        assert_eq!(owner, roles.deployer_addr);

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Telephone::new(self.address, deployer);

        println!("Checking that you became the owner...");
        let owner = contract.owner().call().await?;
        let is_owner = owner == roles.offender_addr;

        Ok(is_owner)
    }
}
