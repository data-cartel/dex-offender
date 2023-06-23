use ethers::prelude::*;

pub mod damn_vulnerable_defi;
pub mod ethernaut;
pub mod level;
pub mod roles;

pub use damn_vulnerable_defi::*;
pub use ethernaut::*;
pub use level::*;
pub use roles::*;

pub fn to_ether<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

pub async fn check_solution<S: Solution>(solution: S) -> eyre::Result<()> {
    println!("\n\n{}", S::Level::DESCRIPTION);

    println!("Initializing accounts...");
    let roles = Roles::new()?;

    println!("Setting up the challenge...");
    let challenge = S::Level::set_up(&roles).await?;

    println!("Running the solution...");
    solution.solve(&challenge, roles.offender.clone()).await?;

    println!("Checking the solution...");
    challenge.check(roles).await?;

    let congratulations = "
~~ $$ !! CONGRATULATIONS !! $$ ~~

Y O U P A S S E D T H E L E V E L

##################################
    ";

    println!("{congratulations}\n");

    Ok(())
}
