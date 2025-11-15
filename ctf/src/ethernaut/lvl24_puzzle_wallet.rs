use alloy::primitives::{keccak256, Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::{puzzle_proxy::PuzzleProxy, puzzle_wallet::PuzzleWallet};
use crate::{roles::*, to_ether, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level24)
    }

    fn name(&self) -> &'static str { "PuzzleWallet" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_addr: _, offender: _, offender_addr: _, some_user: _, some_user_addr: _ } = roles;

        println!("Deploying the PuzzleWallet contract...");
        let contract =
            PuzzleWallet::deploy(deployer).await?;

        let data = keccak256("init(uint256)")
            .into_iter()
            .take(4)
            .chain({
                let mut buff = [0u8; 32];
                to_ether(100).to_little_endian(&mut buff);
                buff
            })
            .collect::<Vec<u8>>();
        println!("data: {:?}", data);
        let proxy = PuzzleProxy::deploy(
            deployer,
            (roles.deployer_addr, *contract.address(), data.into()),
        ).await?;
        let contract2 = PuzzleWallet::new(*proxy.address(), deployer);

        let pending = contract2.add_to_whitelist(roles.deployer_addr).send().await?;
        let _receipt = pending.get_receipt().await?;

        let pending = contract2.deposit().value(U256::from(100_000_000_000_u128)).send().await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, deployer_addr: _, offender, offender_addr: _, some_user: _, some_user_addr: _ } = roles;
        let contract = PuzzleProxy::new(self.address, deployer);
        println!("Checking that you have become the admin of the contract...");

        Ok(contract.admin().call().await? == roles.offender_addr)
    }
}
