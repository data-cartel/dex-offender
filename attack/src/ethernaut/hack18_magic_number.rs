use crate::abi::n::N;
use async_trait::async_trait;
use ctf::ethernaut::lvl18_magic_number::*;
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
        let contract = MagicNum::new(target.address, offender.clone());
        let hack_contract = N::deploy(offender.to_owned(), ())?.send().await?;
        contract.set_solver(hack_contract.address()).send().await?;
        Ok(())
    }
}
