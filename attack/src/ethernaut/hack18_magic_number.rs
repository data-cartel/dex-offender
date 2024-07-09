use async_trait::async_trait;
use ctf::ethernaut::lvl18_magic_number::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 18: Magic Number
     *
     * To solve this level, you only need to provide the
     * Ethernaut with a `Solver`, a contract that
     * responds to `whatIsTheMeaningOfLife()` with the
     * right number.
     *
     * Easy right?
     * Well... there's a catch.
     * The solver's code needs to be really tiny. Really
     * reaaaaaallly tiny. Like freakin' really
     * really itty-bitty tiny: 10 opcodes at most.
     *
     * Hint: Perhaps its time to leave the comfort of
     * the Solidity compiler momentarily, and build this
     * one by hand O_o.
     *
     * That's right: Raw EVM bytecode.
     * Good luck!
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
