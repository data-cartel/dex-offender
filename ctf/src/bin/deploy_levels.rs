use alloy::node_bindings::Anvil;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // default port is 8545 but we don't want to clash if Anvil
    // is already running
    println!("Spawning Anvil...");
    let anvil = Anvil::new()
        .port(8901u16)
        .args(["--dump-state", "state.json", "--state-interval", "1"])
        .spawn();

    println!("Connecting to Anvil...");
    let rpc_url = anvil.endpoint();

    ctf::deploy(&rpc_url, "ctfs.json").await
}
