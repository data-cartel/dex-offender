use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::{
    library_contract::LibraryContract, preservation::Preservation,
};
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level16)
    }

    fn name(&self) -> &'static str { "Preservation" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Preservation contract...");
        let timezone1 =
            LibraryContract::deploy(deployer.as_ref(), ()).await?;
        let timezone2 =
            LibraryContract::deploy(deployer.as_ref(), ()).await?;
        let contract = Preservation::deploy(
            deployer.as_ref(),
            (*timezone1.address(), *timezone2.address()),
        ).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Preservation::new(self.address, deployer.as_ref());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?._0;
        Ok(owner == offender.default_signer_address())
    }
}
