use alloy::{
    network::EthereumWallet,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
            NonceFiller, WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
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
    RootProvider<Http<Client>>,
    Http<Client>,
    alloy::network::Ethereum,
>;

#[derive(Clone)]
pub struct Roles {
    pub deployer: ActorProvider,
    pub some_user: ActorProvider,
    pub offender: ActorProvider,
}

impl Roles {
    pub fn new(rpc_url: &str) -> eyre::Result<Self> {
        let deployer: PrivateKeySigner =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
        let deployer = mk_signer(rpc_url, deployer)?;

        let some_user: PrivateKeySigner =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;
        let some_user = mk_signer(rpc_url, some_user)?;

        let offender: PrivateKeySigner =
            "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".parse()?;
        let offender = mk_signer(rpc_url, offender)?;

        Ok(Roles {
            deployer,
            some_user,
            offender,
        })
    }
}

fn mk_signer(rpc_url: &str, signer: PrivateKeySigner) -> eyre::Result<ActorProvider> {
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);
    Ok(provider)
}
