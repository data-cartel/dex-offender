use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;
use std::str::FromStr;

pub use crate::abi::{
    crypto_vault::CryptoVault, delegate_erc20::DelegateERC20,
    double_entry_point::DoubleEntryPoint,
    double_entry_point_check::DoubleEntryPointCheck, forta::Forta,
    legacy_token::LegacyToken,
};


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level26)
    }

    fn name(&self) -> &'static str { "DoubleEntryPoint" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, some_user: _ } = roles;

        println!("Deploying the DoubleEntryPoint contract...");

        let old_token =
            LegacyToken::deploy(deployer.to_owned(), ())?.send().await?;
        let forta = Forta::deploy(deployer.to_owned(), ())?.send().await?;
        let vault =
            CryptoVault::deploy(deployer.to_owned(), offender.address())?
                .send()
                .await?;
        let new_token = DoubleEntryPoint::deploy(
            deployer.to_owned(),
            (
                old_token.address(),
                vault.address(),
                forta.address(),
                offender.address(),
            ),
        )?
        .send()
        .await?;

        vault.set_underlying(new_token.address()).send().await?;
        old_token.delegate_to_new_contract(new_token.address()).send().await?;
        old_token
            .mint(vault.address(), U256::from(100000000000000000000_u128))
            .send()
            .await?;

        let contract = new_token;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender, some_user: _ } = roles;
        let contract = DoubleEntryPoint::new(self.address, deployer.clone());
        let forta_address = contract.forta().await?;
        let forta = Forta::new(forta_address, deployer.clone());
        let users_detection_bot =
            forta.users_detection_bots(offender.address()).await?;

        if users_detection_bot
            == H160::from_str("0x0000000000000000000000000000000000000000")
                .unwrap()
        {
            println!("Bot not assigned");
            return Ok(false);
        }
        println!("Checking that the tokens are not stolen from the account...");
        let checker = DoubleEntryPointCheck::deploy(
            deployer.to_owned(),
            contract.address(),
        )?
        .send()
        .await?;
        checker.sweep().send().await?;
        checker.checker().send().await?;

        Ok(checker.result().await?)
    }
}
