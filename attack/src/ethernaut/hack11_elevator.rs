use async_trait::async_trait;
use ctf::ethernaut::lvl11_elevator::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 11: Elevator
     *
     * This elevator won't let you reach the top of your
     * building. Right?
     *
     * Things that might help:
     * Sometimes solidity is not good at keeping
     * promises. This Elevator expects to be used
     * from a Building.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
