use async_trait::async_trait;
use ctf::*;
// use ethers::prelude::*;

struct DamnVulnerableDeFiLvl1Exploit;

#[async_trait]
impl Exploit for DamnVulnerableDeFiLvl1Exploit {
    type Level = DamnVulnerableDeFiTarget1;

    async fn solve(
        self,
        _target: &Self::Level,
        _offender: Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}
