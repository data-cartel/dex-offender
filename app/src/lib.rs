use async_trait::async_trait;
use ethers::prelude::*;

use dex_offenders::{DexOffender, DexOffenders};

pub fn to_scaled_u256<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

#[async_trait]
pub trait Level {
    async fn set_up(offenders: &DexOffenders) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn solve(&self, player: &DexOffender) -> eyre::Result<()>;

    async fn validate(self, offenders: &DexOffenders) -> eyre::Result<Self>
    where
        Self: Sized;
}

pub mod damn_vulnerable_defi;
pub mod dex_offenders;
pub mod ethernaut;

#[cfg(test)]
pub async fn test_level<L: Level>() -> eyre::Result<L> {
    let offenders = DexOffenders::init_with_anvil()?;

    let level = L::set_up(&offenders).await?;
    level.solve(&offenders.player).await?;
    level.validate(&offenders).await
}
