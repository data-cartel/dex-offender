use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{
    coin::Coin, good_samaritan::GoodSamaritan, wallet::Wallet,
};


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level27)
    }

    fn name(&self) -> &'static str { "GoodSamaritan" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the GoodSamaritan contract...");

        let contract =
            GoodSamaritan::deploy(deployer.to_owned(), ())?.send().await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = GoodSamaritan::new(self.address, deployer.clone());
        let coin = Coin::new(contract.coin().await?, deployer.clone());
        let wallet = Wallet::new(contract.wallet().await?, deployer.clone());

        Ok(coin.balances(wallet.address()).await? == 0.into())
    }
}
