use async_trait::async_trait;
use bindings::coin_flip::CoinFlip;
use bindings::coin_flip_exploit::CoinFlipExploit;

struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::EthernautLevel3;

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

    // #[ignore]
    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        ctf::check_solution(Solution).await
    }
}
