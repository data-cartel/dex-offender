use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::denial::Denial;
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level20)
    }

    fn name(&self) -> &'static str { "Denial" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Denial contract...");
        let contract = Denial::deploy(deployer.as_ref(), ()).await?;

        let tx = TransactionRequest::default()
            .to(*contract.address())
            .value(U256::from(1_000_000));
        let pending = deployer.send_transaction(tx).await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = Denial::new(self.address, deployer.as_ref());
        println!("Checking that the contract has more than 100 wei...");
        let hundred = U256::from(100_u8);
        if deployer.get_balance(*contract.address()).await? <= hundred {
            // cheating otherwise
            return Ok(false);
        }
        println!("Checking that the owner cannot call withdraw()...");
        let pending = contract.withdraw().gas(1_000_000).send().await?;
        let receipt = pending.get_receipt().await?;

        return Ok(!receipt.status());
    }
}
