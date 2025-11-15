use alloy::primitives::{keccak256, Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;
use rand::Rng;

pub use crate::abi::vault::Vault;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level08)
    }

    fn name(&self) -> &'static str { "Vault" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, .. } = roles;

        println!("Deploying the Vault contract...");
        let random = rand::thread_rng().gen::<[u8; 32]>();
        let psswd = keccak256(random);
        let vault = Vault::deploy(deployer, psswd.into()).await?;

        let target = Target { address: *vault.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Vault::new(self.address, deployer);

        println!("Checking that the contract is unlocked...");
        let unlocked = !contract.locked().call().await?._0;

        Ok(unlocked)
    }
}
