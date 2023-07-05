use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;
use rand::Rng;

pub use bindings::vault::Vault;
use ethers::utils::keccak256;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub contract_address: Address,
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
        let vault = Vault::deploy(deployer.to_owned(), psswd)?.send().await?;

        let target = Target { contract_address: vault.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Vault::new(self.contract_address, deployer.clone());

        println!("Checking that the contract is unlocked...");
        let unlocked = !contract.locked().await?;

        Ok(unlocked)
    }
}
