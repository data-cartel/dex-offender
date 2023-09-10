use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::denial::Denial;

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
        let contract = Denial::deploy(deployer.to_owned(), ())?.send().await?;

        deployer
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .value(1_000_000),
                None,
            )
            .await?
            .await?;

        let target = Target { address: contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = Denial::new(self.address, deployer.clone());
        println!("Checking that the contract has more than 100 wei...");
        let hundred = U256::from(100_u8);
        if deployer.get_balance(contract.address(), None).await? <= hundred {
            // cheating otherwise
            return Ok(false);
        }
        println!("Checking that the owner cannot call withdraw()...");
        let tx = contract.withdraw().gas(1_000_000).send().await?.await?;

        if let Some(receipt) = tx {
            if let Some(status) = receipt.status {
                return Ok(status == 0.into());
            }
        }

        Ok(false)
    }
}
