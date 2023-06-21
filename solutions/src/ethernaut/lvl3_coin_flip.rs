use async_trait::async_trait;
use bindings::coin_flip::CoinFlip;
use bindings::coin_flip_exploit::CoinFlipExploit;
use ctf::*;

struct EthernautLevel3Solution;

#[async_trait]
impl Solution for EthernautLevel3Solution {
    type Level = EthernautLevel3;

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
        ctf::check_solution(EthernautLevel3Solution).await
    }
}
