use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::magic_num::MagicNum;
use crate::{abi::meaning_of_life::MeaningOfLife, roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level18)
    }

    fn name(&self) -> &'static str { "Magic Number" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Magic Number contract...");
        let contract =
            MagicNum::deploy(deployer.as_ref(), ()).await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = MagicNum::new(self.address, deployer.as_ref());
        println!("Verifying that the solver variable is not empty...");
        let hack_contract_address = contract.solver().call().await?._0;
        println!("Check if TheMeaningOfLife() is 42...");
        let hack_contract =
            MeaningOfLife::new(hack_contract_address, deployer.as_ref());
        let magic = hack_contract.what_is_the_meaning_of_life().call().await;
        match magic {
            Err(_) => {
                return Ok(false);
            }
            Ok(magic) => {
                let ft = U256::from(42_u8);
                if magic._0 != ft {
                    println!("It's not 42");
                    return Ok(false);
                }

                println!("Check if the contract size is less than 10 bytes...");
                // Retrieve the contract bytecode
                let bytecode =
                    deployer.get_code_at(hack_contract_address).await?;

                // Get the size of the bytecode in bytes
                let bytecode_size = bytecode.len();
                if bytecode_size > 10 {
                    println!("It's not");
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}
