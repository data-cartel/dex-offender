use async_trait::async_trait;
use bindings::telephone_exploit::TelephoneExploit;
use ctf::ethernaut::lvl04_telephone::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 4: Telephone
     *
     * The goal of this level is for you to claim
     * ownership of the instance you are given.
     *
     * Things that might help
     * - Look into Solidity's documentation on the
     *   delegatecall low level function, how it works,
     *   how it can be used to delegate operations to
     *   on-chain libraries, and what implications it
     *   has on execution scope.
     * - Fallback methods
     * - Method ids
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
