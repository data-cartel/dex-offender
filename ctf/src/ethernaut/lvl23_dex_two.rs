use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::{dex_two::DexTwo, swappable_token_two::SwappableTokenTwo};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level23)
    }

    fn name(&self) -> &'static str { "DexTwo" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, some_user: _ } = roles;

        println!("Deploying the DexTwo contract...");
        let contract = DexTwo::deploy(deployer.to_owned(), ())?.send().await?;
        let token1 = SwappableTokenTwo::deploy(
            deployer.to_owned(),
            (
                contract.address(),
                String::from("Token 1"),
                String::from("TKN1"),
                U256::from(110),
            ),
        )?
        .send()
        .await?;
        let token2 = SwappableTokenTwo::deploy(
            deployer.to_owned(),
            (
                contract.address(),
                String::from("Token 2"),
                String::from("TKN2"),
                U256::from(110),
            ),
        )?
        .send()
        .await?;

        contract.set_tokens(token1.address(), token2.address()).send().await?;
        token1.approve(contract.address(), U256::from(100)).send().await?;
        token2.approve(contract.address(), U256::from(100)).send().await?;

        contract
            .add_liquidity(token1.address(), U256::from(100))
            .send()
            .await?;
        contract
            .add_liquidity(token2.address(), U256::from(100))
            .send()
            .await?;

        token1
            .transfer(offender.address(), U256::from(10))
            .send()
            .await?
            .await?;
        token2
            .transfer(offender.address(), U256::from(10))
            .send()
            .await?
            .await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = DexTwo::new(self.address, deployer.clone());
        println!("Checking that you have stolen all tokens of both types...");

        let token1 = SwappableTokenTwo::new(
            contract.token_1().await?,
            deployer.to_owned(),
        );
        let token2 = SwappableTokenTwo::new(
            contract.token_2().await?,
            deployer.to_owned(),
        );

        Ok(token1.balance_of(contract.address()).await? == 0.into()
            && token2.balance_of(contract.address()).await? == 0.into())
    }
}
