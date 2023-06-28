use ethers::prelude::*;

// pub mod damn_vulnerable_defi;
pub mod ethernaut;
pub mod level;
pub mod roles;

// pub use damn_vulnerable_defi::*;
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
    roles: &Roles,
    solution: S,
) -> eyre::Result<()> {
    let level = S::Level::from_file()?;

    if level.check(roles).await? {
        return Ok(());
    }
    println!("\n\n{}", S::Level::DESCRIPTION);

    println!("Running the solution...");
    solution.solve(&level, &roles.offender).await?;

    println!("Checking the solution...");
    let solved = level.check(roles).await?;
    assert!(solved);

    let congratulations = "
-------------------------------------
//////     CONGRATULATIONS     //////

$$$  Y O U  H A V E  S O L V E D  $$$
$$$   T H E   C H A L L E N G E   $$$

youpassedthelevelyoupassedthelevelyou
passedthelevelyoupassedthelevelyoupas
sedthelevelyoupassedthelevelyoupassed
-------------------------------------
    ";
    println!("{congratulations}\n");

    Ok(())
}
