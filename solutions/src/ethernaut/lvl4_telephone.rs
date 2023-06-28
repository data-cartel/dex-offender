use async_trait::async_trait;
use bindings::telephone_exploit::TelephoneExploit;

pub(crate) struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::Level4;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
