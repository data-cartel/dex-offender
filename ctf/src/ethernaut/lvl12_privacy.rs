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
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Privacy contract...");

        let mk_element = || {
            let random = rand::thread_rng().gen::<[u8; 32]>();
            keccak256(random).into()
        };

        let data = [mk_element(), mk_element(), mk_element()];

        let contract =
            Privacy::deploy(deployer.as_ref(), data).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Privacy::new(self.address, deployer.as_ref());

        println!("Checking that you became the owner...");
        let locked = contract.locked().call().await?._0;

        Ok(!locked)
    }
}
