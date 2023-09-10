use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{puzzle_proxy::PuzzleProxy, puzzle_wallet::PuzzleWallet};

pub use crate::abi::zalupa::Zalupa;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level24)
    }

    fn name(&self) -> &'static str { "PuzzleWallet" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the PuzzleWallet contract...");
        let contract_wallet =
            PuzzleWallet::deploy(deployer.to_owned(), ())?.send().await?;
        let zalupa = Zalupa::deploy(deployer.to_owned(), ())?.send().await?;

        let proxy = PuzzleProxy::deploy(
            deployer.to_owned(),
            (
                deployer.address(),
                contract_wallet.address(),
                zalupa.data().await?,
            ),
        )?
        .send()
        .await?;
        let contract = PuzzleWallet::new(proxy.address(), deployer.to_owned());

        contract.add_to_whitelist(deployer.address()).send().await?;
        contract.deposit().value(100_000_000_000_u128).send().await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = PuzzleProxy::new(self.address, deployer.clone());
        println!("Checking that you have become the admin of the contract...");
        Ok(contract.admin().await? == offender.address())
    }
}
