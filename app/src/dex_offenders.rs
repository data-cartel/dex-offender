use crate::to_scaled_u256;
use ethers::prelude::*;
use ethers::utils::Anvil;
use std::sync::Arc;

pub(crate) type DexOffender = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

#[derive(Debug, Clone)]
pub(crate) struct DexOffenders {
    pub(crate) deployer: DexOffender,
    pub(crate) player: DexOffender,
    pub(crate) some_user: DexOffender,
}

impl DexOffenders {
    pub(crate) fn init_without_anvil() -> eyre::Result<DexOffenders> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        let deployer: LocalWallet =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer = mk_offender(provider.clone(), deployer, Chain::AnvilHardhat)?;

        let player: LocalWallet =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let player = mk_offender(provider.clone(), player, Chain::AnvilHardhat)?;

        let some_user: LocalWallet =
            "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a".parse()?;
        let some_user = mk_offender(provider.clone(), some_user, Chain::AnvilHardhat)?;

        Ok(DexOffenders { deployer, player, some_user })
    }

    pub(crate) fn init_with_anvil() -> eyre::Result<DexOffenders> {
        let anvil = Anvil::new().spawn();

        let provider = Provider::<Http>::try_from(anvil.endpoint())?;

        let deployer: LocalWallet = anvil.keys()[0].clone().into();
        let player: LocalWallet = anvil.keys()[1].clone().into();
        let some_user: LocalWallet = anvil.keys()[2].clone().into();

        let deployer = mk_offender(provider.clone(), deployer, anvil.chain_id())?;
        let player = mk_offender(provider.clone(), player, anvil.chain_id())?;
        let some_user = mk_offender(provider.clone(), some_user, anvil.chain_id())?;

        Ok(DexOffenders { deployer, player, some_user })
    }
}

fn mk_offender(
    provider: Provider<Http>,
    wallet: LocalWallet,
    chain_id: impl Into<u64>,
) -> eyre::Result<DexOffender> {
    let wallet = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Ok(Arc::new(wallet))
}
