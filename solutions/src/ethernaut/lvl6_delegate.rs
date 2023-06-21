use async_trait::async_trait;
use ctf::*;
use ethers::prelude::*;
use ethers::providers::Middleware;
use ethers::utils::keccak256;

pub struct EthernautLevel6Solution;

#[async_trait]
impl Solution for EthernautLevel6Solution {
    type Level = EthernautLevel6;

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

    #[ignore]
    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        ctf::check_solution(EthernautLevel6Solution).await
    }
}
