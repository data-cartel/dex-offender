use async_trait::async_trait;
use ctf::*;
// use ethers::prelude::*;

struct DamnVulnerableDeFiLvl1Solution;

#[async_trait]
impl Solution for DamnVulnerableDeFiLvl1Solution {
    type Level = DamnVulnerableDeFiChallenge1;

    async fn solve(
        self,
        _challenge: &Self::Level,
        _offender: Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
