use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{
    library_contract::LibraryContract, preservation::Preservation,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level17)
    }

    fn name(&self) -> &'static str { "Recovery" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, some_user: _ } = roles;

        println!("Deploying the Recovery contract...");
        let contract =
            Recovery::deploy(deployer.to_owned(), ())?.send().await?;
        contract.generate_token("InitialToken", deployer.address() , U256::from(100000_u8)).send().await?;


        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = Recovery::new(self.address, deployer.clone());

        println!("...");

        address(uint160(uint256(keccak256(abi.encodePacked(uint8(0xd6), uint8(0x94), recoveryInstance, uint8(0x01))))));

        let owner = contract.owner().await?;
        Ok(owner == offender.address())
    }
}
