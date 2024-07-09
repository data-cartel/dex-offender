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
        Ok(())
    }
}
