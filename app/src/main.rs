use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use ::app::*;
use bindings::damn_valuable_token::DamnValuableToken;
use bindings::receiver_unstoppable::ReceiverUnstoppable;
use bindings::unstoppable_vault::UnstoppableVault;

#[tokio::main]
async fn main() -> Result<()> {
    let tokens_in_vault = to_scaled_u256(1_000_000);
    let initial_player_token_balance = to_scaled_u256(10);
    let flash_loan_amount = to_scaled_u256(100);

    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let provider = Arc::new(provider);

    let deployer: LocalWallet =
        "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
    let deployer = deployer.with_chain_id(Chain::AnvilHardhat);

    let deployer = SignerMiddleware::new(provider.clone(), deployer);
    let deployer = Arc::new(deployer);

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

        // tokens_in_vault - 1u64.into()
        let amount: U256 = tokens_in_vault - U256::from(1u64);
        let flash_fee = vault.flash_fee(token.address(), amount).await?;
        assert_eq!(flash_fee, 0u64.into());

        let flash_fee = vault.flash_fee(token.address(), tokens_in_vault).await?;
        assert_eq!(flash_fee, to_scaled_u256(50_000));
    }

    let player: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;

    token.transfer(player, initial_player_token_balance).send().await?.await?;

    {
        let balance = token.balance_of(player).await?;
        assert_eq!(balance, initial_player_token_balance);
    }

    let some_user: LocalWallet =
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
    let some_user = some_user.with_chain_id(Chain::AnvilHardhat);

    let some_user = SignerMiddleware::new(provider, some_user);
    let some_user = Arc::new(some_user);

    let receiver = ReceiverUnstoppable::deploy(some_user.clone(), vault.address())?;
    let receiver = receiver.send().await?;
    receiver.execute_flash_loan(flash_loan_amount).send().await?.await?;

    Ok(())
}
