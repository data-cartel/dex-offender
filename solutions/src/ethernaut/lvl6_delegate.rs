use async_trait::async_trait;
use ethers::prelude::*;
use ethers::providers::Middleware;
use ethers::utils::keccak256;

pub(crate) struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::Level6;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
