use async_trait::async_trait;
use ethers::prelude::*;

use crate::roles::*;
use crate::{to_ether, Target};
use bindings::damn_valuable_token::DamnValuableToken;
use bindings::receiver_unstoppable::ReceiverUnstoppable;
use bindings::unstoppable_vault::UnstoppableVault;

pub struct DamnVulnerableDeFiTarget1 {
    pub vault: UnstoppableVault<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub token: DamnValuableToken<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub receiver:
        ReceiverUnstoppable<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[async_trait]
impl Target for DamnVulnerableDeFiTarget1 {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.target)
    }

    async fn set_up(roles: &Roles) -> eyre::Result<DamnVulnerableDeFiTarget1> {
        let Roles { deployer, offender, some_user } = roles;

        let tokens_in_vault = to_ether(1_000_000);
        let initial_offender_token_balance = to_ether(10);
        let flash_loan_amount = to_ether(100);

        let token = DamnValuableToken::deploy(deployer.clone(), ())?;
        let token = token.send().await?;

        let vault = UnstoppableVault::deploy(
            deployer.clone(),
            (token.address(), deployer.address(), deployer.address()),
        )?;
        let vault = vault.send().await?;

        {
            let asset = vault.asset().await?;
            assert_eq!(asset, token.address());
        }

        token.approve(vault.address(), tokens_in_vault).send().await?.await?;
        vault
            .deposit(tokens_in_vault, deployer.address())
            .send()
            .await?
            .await?;

        {
            let balance = token.balance_of(vault.address()).await?;
            assert_eq!(balance, tokens_in_vault);

            let total_assets = vault.total_assets().await?;
            assert_eq!(total_assets, tokens_in_vault);

            let total_supply = vault.total_supply().await?;
            assert_eq!(total_supply, tokens_in_vault);

            let max_flash_loan = vault.max_flash_loan(token.address()).await?;
            assert_eq!(max_flash_loan, tokens_in_vault);

            let amount: U256 = tokens_in_vault - U256::from(1u64);
            let flash_fee = vault.flash_fee(token.address(), amount).await?;
            assert_eq!(flash_fee, 0u64.into());

            let flash_fee =
                vault.flash_fee(token.address(), tokens_in_vault).await?;
            assert_eq!(flash_fee, to_ether(50_000));
        }

        token
            .transfer(offender.address(), initial_offender_token_balance)
            .send()
            .await?
            .await?;

        {
            let balance = token.balance_of(offender.address()).await?;
            assert_eq!(balance, initial_offender_token_balance);
        }

        let receiver =
            ReceiverUnstoppable::deploy(some_user.clone(), vault.address())?;
        let receiver = receiver.send().await?;
        receiver.execute_flash_loan(flash_loan_amount).send().await?.await?;

        let target = DamnVulnerableDeFiTarget1 { vault, token, receiver };

        Ok(target)
    }

    const DESCRIPTION: &'static str = "Damn Vulnerable DeFi
     Target #1 - Unstoppable

    There’s a tokenized vault with a million DVT tokens deposited. It’s \
    offering flash loans for free, until the grace period ends.

    To pass the target, make the vault stop offering flash loans.

    You start with 10 DVT tokens in balance.
    ";

    async fn check(
        self,
        _offender: Roles,
    ) -> eyre::Result<DamnVulnerableDeFiTarget1> {
        unimplemented!("check function not implemented for this target")
    }
}
