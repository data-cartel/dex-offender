use async_trait::async_trait;
use ctf::ethernaut::lvl09_king::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 9: King
     *
     * The contract below represents a very simple game:
     * whoever sends it an amount of ether that is
     * larger than the current prize becomes the new
     * king. On such an event, the overthrown king
     * gets paid the new prize, making a bit of ether
     * in the process! As ponzi as it gets xD
     *
     * Such a fun game. Your goal is to break it.
     *
     * When you submit the instance back to the level,
     * the level is going to reclaim kingship. You
     * will beat the level if you can avoid such a self
     * proclamation.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        todo!("Solve me!")
    }
}