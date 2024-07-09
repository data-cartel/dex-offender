use async_trait::async_trait;
use ctf::ethernaut::lvl01_fallback::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     *  @title Ethernaut Level 1: Fallback
     *
     *  Look carefully at the contract's code below.
     *
     *  You will beat this target if
     *  1. you claim ownership of the contract
     *  2. you reduce its balance to 0
     *
     *  Things that might help:
     *  - How to send ether when interacting with an ABI
     *  - How to send ether outside of the ABI
     *  - Fallback methods
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
