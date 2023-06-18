use async_trait::async_trait;
use ethers::prelude::*;

use crate::dex_offenders::*;
use crate::{to_scaled_u256, Level};
use bindings::damn_valuable_token::DamnValuableToken;
use bindings::receiver_unstoppable::ReceiverUnstoppable;
use bindings::unstoppable_vault::UnstoppableVault;

pub struct DamnVulnerableDeFiLevel1 {
    pub vault: UnstoppableVault<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub token: DamnValuableToken<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub receiver: ReceiverUnstoppable<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[async_trait]
impl Level for DamnVulnerableDeFiLevel1 {
    async fn set_up(offenders: &DexOffenders) -> eyre::Result<DamnVulnerableDeFiLevel1> {
        let DexOffenders { deployer, player, some_user } = offenders;

        let tokens_in_vault = to_scaled_u256(1_000_000);
        let initial_player_token_balance = to_scaled_u256(10);
        let flash_loan_amount = to_scaled_u256(100);

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
        vault.deposit(tokens_in_vault, deployer.address()).send().await?.await?;

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

            let flash_fee = vault.flash_fee(token.address(), tokens_in_vault).await?;
            assert_eq!(flash_fee, to_scaled_u256(50_000));
        }

        token.transfer(player.address(), initial_player_token_balance).send().await?.await?;

        {
            let balance = token.balance_of(player.address()).await?;
            assert_eq!(balance, initial_player_token_balance);
        }

        let receiver = ReceiverUnstoppable::deploy(some_user.clone(), vault.address())?;
        let receiver = receiver.send().await?;
        receiver.execute_flash_loan(flash_loan_amount).send().await?.await?;

        let level = DamnVulnerableDeFiLevel1 { vault, token, receiver };

        Ok(level)
    }

    async fn solve(&self, _player: &DexOffender) -> eyre::Result<()> {
        todo!("Solve me!")
    }

    async fn validate(self, _offenders: &DexOffenders) -> eyre::Result<DamnVulnerableDeFiLevel1> {
        unimplemented!("Validate function not implemented for this level")
    }
}
