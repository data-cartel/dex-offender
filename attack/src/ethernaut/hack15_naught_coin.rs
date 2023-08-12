use async_trait::async_trait;
use ctf::ethernaut::lvl15_naught_coin::*;
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
        let contract = NaughtCoin::new(target.address, offender.clone());

        Ok(())
    }
}
