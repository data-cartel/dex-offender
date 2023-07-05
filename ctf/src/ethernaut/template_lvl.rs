use crate::roles::*;
use crate::Level;
use async_trait::async_trait;
use ethers::prelude::*;

pub use bindings::replaceme::ReplaceMe;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub contract_address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level??)
    }

    fn name(&self) -> &'static str {
        "ReplaceMe"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the ReplaceMe contract...");
        let contract =
            ReplaceMe::deploy(deployer.to_owned(), constructor_args)?
                .send()
                .await?;

        let target = Target { contract_address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = ReplaceMe::new(self.contract_address, deployer.clone());

        println!("Checking that you became the owner...");
        let owner = contract.whatisthis().await?;
        let pass = owner == roles.offender.address();

        Ok(pass)
    }
}
