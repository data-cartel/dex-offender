use async_trait::async_trait;
use ethers::prelude::*;

use roles::{Actor, Roles};

pub fn to_ether<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

#[async_trait]
pub trait Level {
    const DESCRIPTION: &'static str;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn solve(&self, offender: Actor) -> eyre::Result<()>;

    async fn check(self, roles: Roles) -> eyre::Result<Self>
    where
        Self: Sized;
}

pub mod damn_vulnerable_defi;
pub mod ethernaut;
pub mod roles;

#[cfg(test)]
pub async fn test_level<L: Level>() -> eyre::Result<L> {
    println!("\n\n{}", L::DESCRIPTION);

    println!("Setting up roles...");
    let roles = Roles::new()?;

    println!("Setting up the challenge...");
    let level = L::set_up(&roles).await?;

    println!("Running the solution...");
    level.solve(roles.offender.clone()).await?;

    println!("Checking the solution...");
    level.check(roles).await
}
