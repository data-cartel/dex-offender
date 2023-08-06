use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::{prelude::*, utils::keccak256};
use rand::Rng;

pub use crate::abi::privacy::Privacy;

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
            keccak256(random)
        };

        let data = [mk_element(), mk_element(), mk_element()];

        let contract =
            Privacy::deploy(deployer.to_owned(), data)?.send().await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Privacy::new(self.address, deployer.clone());

        println!("Checking that you became the owner...");
        let locked = contract.locked().await?;

        Ok(!locked)
    }
}
