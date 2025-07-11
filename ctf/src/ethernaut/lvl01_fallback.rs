use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;

pub use crate::abi::fallback::Fallback;
use crate::{roles::*, to_ether, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level01)
    }

    fn name(&self) -> &'static str {
        "Fallback"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, deployer_address, offender_address, .. } = roles;

        println!("Deploying the Fallback contract...");
        let contract = Fallback::deploy(deployer, ()).await?;

        let balance =
            contract.contributions(*deployer_address).call().await?._0;
        assert_eq!(balance, to_ether(1000));

        let balance =
            contract.contributions(*offender_address).call().await?._0;
        assert_eq!(balance, U256::from(0));

        let tx = TransactionRequest::default()
            .to(contract.address())
            .value(to_ether(5));

        let receipt =
            deployer.send_transaction(tx).await?.get_receipt().await?;
        println!(
            "Sent initial funding transaction: {:?}",
            receipt.transaction_hash
        );

        let contract_balance = deployer.get_balance(contract.address()).await?;
        assert_eq!(contract_balance, to_ether(5));

        let owner = contract.owner().call().await?._0;
        assert_eq!(owner, *deployer_address);

        let target = Target { address: contract.address() };

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender_address, .. } = roles;
        let contract = Fallback::new(self.address, deployer);

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?._0;
        let is_owner = owner == *offender_address;

        println!("Checking that you reduced its balance to 0...");
        let contract_balance = deployer.get_balance(self.address).await?;
        let balance_reduced = contract_balance == U256::from(0);

        Ok(is_owner && balance_reduced)
    }
}
