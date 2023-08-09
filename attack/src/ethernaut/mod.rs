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
//pub mod hack13_gatekeeper_one;
//pub mod hack14_gatekeeper_two;
//pub mod hack15_naught_coin;
//pub mod hack16_preservation;
//pub mod hack17_recovery;
//pub mod hack18_magic_number;
//pub mod hack19_alien_codex;
//pub mod hack20_denial;
//pub mod hack21_shop;
//pub mod hack22_dex;
pub mod hack23_dex_two;

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
    //use hack13_gatekeeper_one as hack13;
    //use hack14_gatekeeper_two as hack14;
    //use hack15_naught_coin as hack15;
    //use hack16_preservation as hack16;
    //use hack17_recovery as hack17;
    //use hack18_magic_number as hack18;
    //use hack19_alien_codex as hack19;
    //use hack20_denial as hack20;
    //use hack21_shop as hack21;
    //use hack22_dex as hack22;
    use hack23_dex_two as hack23;

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
        //ctf::check_exploit(&roles, hack13::Exploit).await?;
        //ctf::check_exploit(&roles, hack14::Exploit).await?;
        //ctf::check_exploit(&roles, hack15::Exploit).await?;
        //ctf::check_exploit(&roles, hack16::Exploit).await?;
        //ctf::check_exploit(&roles, hack17::Exploit).await?;
        //ctf::check_exploit(&roles, hack18::Exploit).await?;
        //ctf::check_exploit(&roles, hack19::Exploit).await?;
        //ctf::check_exploit(&roles, hack20::Exploit).await?;
        //ctf::check_exploit(&roles, hack21::Exploit).await?;
        //ctf::check_exploit(&roles, hack22::Exploit).await?;
        ctf::check_exploit(&roles, hack23::Exploit).await?;
        Ok(())
    }
}
