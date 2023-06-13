use bindings::::Counter;

use ethers::{prelude::Middleware, providers::test_provider::GOERLI, types::Address};

use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = GOERLI.provider();
    let provider = Arc::new(provider);

    let player = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse::<Address>()?;
    let some_user = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720".parse::<Address>()?;
    let deployer = "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f".parse::<Address>()?;

    let contract = Counter::new(address, provider);
    let blk = contract.client().get_block_number().await?;
    println!("Hello, world! {}", blk);
    Ok(())
}
