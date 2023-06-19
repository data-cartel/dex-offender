use async_trait::async_trait;
use ctf::*;
// use ethers::prelude::*;

pub struct EthernautLevel3Solution;

#[async_trait]
impl Solution for EthernautLevel3Solution {
    type Level = EthernautLevel3;

    async fn solve(
        self,
        _challenge: &Self::Level,
        _offender: Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> eyre::Result<()> {
        check_solution(EthernautLevel3Solution).await
    }
}
