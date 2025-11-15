use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
            NonceFiller, WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
};

pub type ActorProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<
                GasFiller,
                JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
            >,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

#[derive(Clone)]
pub struct Roles {
    pub deployer: ActorProvider,
    pub deployer_addr: Address,
    pub some_user: ActorProvider,
    pub some_user_addr: Address,
    pub offender: ActorProvider,
    pub offender_addr: Address,
}

impl Roles {
    pub fn new(rpc_url: &str) -> eyre::Result<Self> {
        let deployer_signer: PrivateKeySigner =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer_addr = deployer_signer.address();
        let deployer = mk_signer(rpc_url, deployer_signer)?;

        let some_user_signer: PrivateKeySigner =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user_addr = some_user_signer.address();
        let some_user = mk_signer(rpc_url, some_user_signer)?;

        let offender_signer: PrivateKeySigner =
            "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
        let offender_addr = offender_signer.address();
        let offender = mk_signer(rpc_url, offender_signer)?;

        Ok(Roles {
            deployer,
            deployer_addr,
            some_user,
            some_user_addr,
            offender,
            offender_addr,
        })
    }
}

fn mk_signer(rpc_url: &str, signer: PrivateKeySigner) -> eyre::Result<ActorProvider> {
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);
    Ok(provider)
}
