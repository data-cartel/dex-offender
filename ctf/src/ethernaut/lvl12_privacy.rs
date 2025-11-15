use alloy::primitives::{keccak256, Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;
use rand::Rng;

pub use crate::abi::privacy::Privacy;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level12)
    }

    fn name(&self) -> &'static str { "Privacy" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_addr: _, offender: _, offender_addr: _, some_user: _, some_user_addr: _ } = roles;

        println!("Deploying the Privacy contract...");

        let mk_element = || {
            let random = rand::thread_rng().gen::<[u8; 32]>();
            keccak256(random).into()
        };

        let data = [mk_element(), mk_element(), mk_element()];

        let contract =
            Privacy::deploy(deployer, data).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Privacy::new(self.address, deployer);

        println!("Checking that you became the owner...");
        let locked = contract.locked().call().await?;

        Ok(!locked)
    }
}
