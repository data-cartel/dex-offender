use ethers::prelude::*;
use ethers::utils::Anvil;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Spawning Anvil...");
    let anvil =
        Anvil::new().port(8901u16).args(["--dump-state", "state.json"]).spawn();

    println!("Connecting to Anvil...");
    let provider = Provider::<Http>::try_from(anvil.endpoint())?;

    println!("Setting up Ethernaut...");
    let ethernaut = ctf::set_up_ethernaut(provider).await?;

    // println!("Setting up Damn Vulnerable DeFi...");
    // let davu_defi = ctf::set_up_ethernaut(provider).await?;

    println!("Saving levels to a file...");
    let ctfs = ctf::CTFs { ethernaut };
    std::fs::write("ctfs.json", serde_json::to_string_pretty(&ctfs)?)?;

    Ok(())
}
