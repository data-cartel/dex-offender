use ethers::prelude::*;

// pub mod damn_vulnerable_defi;
pub mod abi;
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

pub async fn deploy(provider: Provider<Http>, path: &str) -> eyre::Result<()> {
    println!("Setting up Ethernaut...");
    let ethernaut = set_up_ethernaut(provider).await?;

    // we need to wait for at least a second for Anvil to save
    // the state
    println!("Saving blockchain state to a file...");
    let delay = tokio::time::sleep(std::time::Duration::from_secs(2));

    println!("Saving levels to a file...");
    let ctfs = CTFs { ethernaut };

    std::fs::write(path, serde_json::to_string_pretty(&ctfs)?)?;

    delay.await;

    Ok(())
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

pub async fn check_exploit<E: Exploit>(
    roles: &Roles,
    exploit: E,
) -> eyre::Result<()> {
    let level = E::Target::from_file()?;

    if level.check(roles).await? {
        return Ok(());
    }
    println!("\n\n{}", level.name());

    println!("Running the exploit...");
    exploit.attack(&level, &roles.offender).await?;

    println!("Checking the exploit...");
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
