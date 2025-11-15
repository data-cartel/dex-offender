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
            Telephone::deploy(deployer.as_ref(), ()).await?;

        let owner = contract.owner().call().await?._0;
        assert_eq!(owner, deployer.default_signer_address());

        let target = Target { address: *contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Telephone::new(self.address, deployer.as_ref());

        println!("Checking that you became the owner...");
        let owner = contract.owner().call().await?._0;
        let is_owner = owner == roles.offender.default_signer_address();

        Ok(is_owner)
    }
}
