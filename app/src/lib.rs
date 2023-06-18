use async_trait::async_trait;
use ethers::prelude::*;

use dex_offenders::{DexOffender, DexOffenders};

pub(crate) fn to_scaled_u256<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

#[async_trait]
trait Level {
    async fn set_up(&self, offenders: DexOffenders) -> eyre::Result<()>;

    async fn solve(&self, player: DexOffender) -> eyre::Result<()>;

    async fn validate(&self) -> eyre::Result<()>;
}

pub(crate) mod damn_vulnerable_defi;
pub(crate) mod dex_offenders;

#[cfg(test)]
pub(crate) async fn test_level<L: Level>(level: L) -> eyre::Result<()> {
    let offenders = DexOffenders::init_with_anvil()?;
    level.set_up(offenders.clone()).await?;

    let player = offenders.player;
    level.solve(player).await?;

    level.validate().await?;

    Ok(())
}
