use async_trait::async_trait;
use ctf::ethernaut::lvl12_privacy::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 12: Privacy
     *
     * The creator of this contract was careful enough
     * to protect the sensitive areas of its storage.
     *
     * Unlock this contract to beat the level.
     *
     * Things that might help:
     * Understanding how storage works
     * Understanding how parameter parsing works
     * Understanding how casting works
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
