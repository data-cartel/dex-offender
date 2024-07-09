use async_trait::async_trait;
use ctf::ethernaut::lvl05_token::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 5: Token
     *
     * The goal of this level is for you to hack the
     * basic token contract below.
     *
     * You are given 20 tokens to start with and you
     * will beat the level if you somehow manage to
     * get your hands on any additional tokens.
     * Preferably a very large amount of tokens.
     *
     * Things that might help:
     * - What is an odometer?
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
