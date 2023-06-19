use async_trait::async_trait;
use ethers::prelude::*;

pub mod damn_vulnerable_defi;
pub mod ethernaut;
pub mod roles;

pub use damn_vulnerable_defi::*;
pub use ethernaut::*;
pub use roles::*;

pub fn to_ether<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

#[async_trait]
pub trait Challenge {
    const DESCRIPTION: &'static str;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn check(self, roles: Roles) -> eyre::Result<Self>
    where
        Self: Sized;
}

#[async_trait]
pub trait Solution {
    type Level: Challenge;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: Actor,
    ) -> eyre::Result<()>;
}

pub async fn check_solution<S: Solution>(solution: S) -> eyre::Result<()> {
    println!("\n\n{}", S::Level::DESCRIPTION);

    println!("Setting up roles...");
    let roles = Roles::new()?;

    println!("Setting up the challenge...");
    let challenge = S::Level::set_up(&roles).await?;

    println!("Running the solution...");
    solution.solve(&challenge, roles.offender.clone()).await?;

    println!("Checking the solution...");
    challenge.check(roles).await?;

    Ok(())
}
