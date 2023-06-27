use ethers::prelude::*;

pub mod damn_vulnerable_defi;
pub mod ethernaut;
pub mod level;
pub mod roles;

pub use damn_vulnerable_defi::*;
pub use ethernaut::*;
pub use level::*;
pub use roles::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CTFs {
    pub ethernaut: Ethernaut,
    // pub damn_vulnerable_defi: DamnVulnerableDeFi,
}

impl CTFs {
    pub fn from_file() -> eyre::Result<Self> {
        let ctf_json = std::fs::read_to_string("../ctfs.json")?;
        let ctf: CTFs = serde_json::from_str(&ctf_json)?;
        Ok(ctf)
    }
}

pub fn to_ether<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}

pub async fn check_solution<S: Solution>(
    challenge: S::Level,
    solution: S,
) -> eyre::Result<()> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    println!("\n\n{}", S::Level::DESCRIPTION);

    println!("Initializing accounts...");
    let roles = Roles::new(provider)?;

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
