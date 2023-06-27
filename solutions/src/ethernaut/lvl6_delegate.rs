use async_trait::async_trait;
use ethers::prelude::*;
use ethers::providers::Middleware;
use ethers::utils::keccak256;

pub struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::Level6;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: ctf::Actor,
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
        ctf::check_solution(Solution).await
    }
}
