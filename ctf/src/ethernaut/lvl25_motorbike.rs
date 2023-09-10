use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;
use std::str::FromStr;

pub use crate::abi::{engine::Engine, motorbike::Motorbike};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level25)
    }

    fn name(&self) -> &'static str { "Motorbike" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Motorbike contract...");
        let contract_engine =
            Engine::deploy(deployer.to_owned(), ())?.send().await?;
        let contract =
            Motorbike::deploy(deployer.to_owned(), contract_engine.address())?
                .send()
                .await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let bike = Motorbike::new(self.address, deployer.clone());
        let memory_slot = String::from("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");
        let hash = H256::from_str(&memory_slot);
        let engine = offender
            .get_storage_at(bike.address(), hash.unwrap(), None)
            .await?;
        let contract = Engine::new(engine, deployer.clone());
        let code = deployer.get_code(contract.address(), None).await?;

        Ok(code == Bytes::from_str("0x").unwrap())
    }
}
