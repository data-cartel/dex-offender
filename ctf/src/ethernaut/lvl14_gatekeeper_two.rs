use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::gatekeeper_two::GatekeeperTwo;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level14)
    }

    fn name(&self) -> &'static str { "GatekeeperTwo" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_addr: _, offender: _, offender_addr: _, some_user: _, some_user_addr: _ } = roles;

        println!("Deploying the GatekeeperTwo contract...");
        let contract =
            GatekeeperTwo::deploy(deployer).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = GatekeeperTwo::new(self.address, deployer);

        println!("Checking the entrant...");
        let entrant = contract.entrant().call().await?;
        let pass = entrant == roles.offender_addr;

        Ok(pass)
    }
}
