use async_trait::async_trait;
use bindings::coin_flip::CoinFlip;
use bindings::coin_flip_exploit::CoinFlipExploit;

pub(crate) struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::Level3;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
