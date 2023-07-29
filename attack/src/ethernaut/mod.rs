//pub mod hack01_fallback;
//pub mod hack02_fallout;
//pub mod hack03_coin_flip;
//pub mod hack04_telephone;
//pub mod hack05_token;
//pub mod hack06_delegate;
//pub mod hack07_force;
//pub mod hack08_vault;
//pub mod hack09_king;
//pub mod hack10_reentrancy;
//pub mod hack11_elevator;
//pub mod hack12_privacy;
pub mod hack13_gatekeeperOne;

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::prelude::*;

    //use hack01_fallback as hack01;
    //use hack02_fallout as hack02;
    //use hack03_coin_flip as hack03;
    //use hack04_telephone as hack04;
    //use hack05_token as hack05;
    //use hack06_delegate as hack06;
    //use hack07_force as hack07;
    //use hack08_vault as hack08;
    //use hack09_king as hack09;
    //use hack10_reentrancy as hack10;
    //use hack11_elevator as hack11;
    //use hack12_privacy as hack12;
    use hack13_gatekeeperOne as hack13;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        println!("Initializing accounts...");
        let roles = ctf::Roles::new(provider)?;

        //ctf::check_exploit(&roles, hack01::Exploit).await?;
        //ctf::check_exploit(&roles, hack02::Exploit).await?;
        //ctf::check_exploit(&roles, hack03::Exploit).await?;
        //ctf::check_exploit(&roles, hack04::Exploit).await?;
        //ctf::check_exploit(&roles, hack05::Exploit).await?;
        //ctf::check_exploit(&roles, hack06::Exploit).await?;
        //ctf::check_exploit(&roles, hack07::Exploit).await?;
        //ctf::check_exploit(&roles, hack08::Exploit).await?;
        //ctf::check_exploit(&roles, hack09::Exploit).await?;
        //ctf::check_exploit(&roles, hack10::Exploit).await?;
        //ctf::check_exploit(&roles, hack11::Exploit).await?;
        //ctf::check_exploit(&roles, hack12::Exploit).await?;
        ctf::check_exploit(&roles, hack13::Exploit).await?;

        Ok(())
    }
}
