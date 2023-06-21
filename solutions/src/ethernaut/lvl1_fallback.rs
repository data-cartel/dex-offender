use async_trait::async_trait;
use ethers::prelude::*;

use bindings::fallback::Fallback;
use ctf::*;

struct EthernautLevel1Solution;

#[async_trait]
impl Solution for EthernautLevel1Solution {
    type Level = EthernautLevel1;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        ctf::check_solution(EthernautLevel1Solution).await
    }
}
