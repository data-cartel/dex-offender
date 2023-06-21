use crate::abi::gatexploit_two::GatexploitTwo;
use async_trait::async_trait;
use ctf::ethernaut::lvl14_gatekeeper_two::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 14: Gatekeeper Two
     *
     * This gatekeeper introduces a few new challenges.
     * Register as an entrant to pass this level.
     *
     * Remember what you've learned from getting past
     * the first gatekeeper
     * - the first gate is the same.
     * The `assembly` keyword in the second gate allows
     * a contract to access functionality that is
     * not native to vanilla Solidity. See [here](http://solidity.readthedocs.io/en/v0.4.23/assembly.html)
     * for more information.
     *
     * The `extcodesize` call in this gate will get the
     * size of a contract's code at a given address
     * * - you can learn more about how and when this
     * is set in section 7 of the
     * [yellow paper](https://ethereum.github.io/yellowpaper/paper.pdf).
     *
     * The `^` character in the third gate is a bitwise
     * operation (XOR), and is used here to apply
     * another common bitwise operation (see [here](http://solidity.readthedocs.io/en/v0.4.23/miscellaneous.html#cheatsheet)).
     *
     * The Coin Flip level is also a good place to start
     * when approaching this challenge.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        GatexploitTwo::deploy(offender.to_owned(), target.address)?
            .send()
            .await?;

        Ok(())
    }
}
