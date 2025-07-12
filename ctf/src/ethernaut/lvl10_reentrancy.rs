use crate::{roles::*, Level};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    transports::http::{Client, Http},
};
use async_trait::async_trait;

pub use crate::abi::reentrance::Reentrance;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level10)
    }

    fn name(&self) -> &'static str {
        "Re-entrancy"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user } = roles;

        println!("Deploying the Reentrance contract...");
        let contract = Reentrance::deploy(deployer, ()).await?;

        deployer
            .send_transaction(
                alloy::rpc::types::TransactionRequest::default()
                    .to(contract.address())
                    .value(U256::from(1e18 as u64)), // 1 ether
            )
            .await?
            .get_receipt()
            .await?;

        contract
            .donate(some_user.address())
            .value(U256::from(20e18 as u64))
            .send()
            .await?
            .get_receipt()
            .await?;

        let contract = Reentrance::new(contract.address(), some_user);
        contract
            .donate(deployer.address())
            .value(U256::from(100e18 as u64))
            .send()
            .await?
            .get_receipt()
            .await?;

        let target = Target { address: contract.address() };
        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, .. } = roles;
        let contract = Reentrance::new(self.address, deployer);

        println!("Checking the contract balance...");
        let balance = deployer.get_balance(contract.address()).await?;

        Ok(balance == U256::ZERO)
    }
}
