use ethers::prelude::*;
use std::sync::Arc;

pub type Actor = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

#[derive(Debug, Clone)]
pub struct Roles {
    pub deployer: Actor,
    pub some_user: Actor,
    pub offender: Actor,
}

impl Roles {
    pub fn new() -> eyre::Result<Self> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        let deployer: LocalWallet =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer =
            mk_signer(provider.clone(), deployer, Chain::AnvilHardhat)?;

        let some_user: LocalWallet =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user =
            mk_signer(provider.clone(), some_user, Chain::AnvilHardhat)?;

        let offender: LocalWallet =
            "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
        let offender = mk_signer(provider, offender, Chain::AnvilHardhat)?;

        Ok(Roles { deployer, some_user, offender })
    }
}

fn mk_signer(
    provider: Provider<Http>,
    wallet: LocalWallet,
    chain_id: impl Into<u64>,
) -> eyre::Result<Actor> {
    let wallet =
        SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Ok(Arc::new(wallet))
}
