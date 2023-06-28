use ethers::prelude::*;
use ethers::utils::Anvil;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Spawning Anvil...");
    // default port is 8545 but we don't want to clash if Anvil is already running
    let anvil = Anvil::new()
        .port(8901u16)
        .args(["--dump-state", "state.json", "--state-interval", "1"])
        .spawn();

    println!("Connecting to Anvil...");
    let provider = Provider::<Http>::try_from(anvil.endpoint())?;

    println!("Setting up Ethernaut...");
    let ethernaut = ctf::set_up_ethernaut(provider).await?;

    // println!("Setting up Damn Vulnerable DeFi...");
    // let davu_defi = ctf::set_up_ethernaut(provider).await?;

    // we need to wait for at least a second for Anvil to save the sate
    println!("Saving blockchain state to a file...");
    let delay = tokio::time::sleep(std::time::Duration::from_secs(2));

    println!("Saving levels to a file...");
    let ctfs = ctf::CTFs { ethernaut };
    std::fs::write("ctfs.json", serde_json::to_string_pretty(&ctfs)?)?;

    delay.await;

    Ok(())
}
