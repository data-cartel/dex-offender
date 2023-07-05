pub mod hack01_fallback;
// pub mod hack02_fallout;
// pub mod hack03_coin_flip;
// pub mod hack04_telephone;
// pub mod hack05_token;
// pub mod hack06_delegate;
// pub mod hack07_force;
// pub mod hack08_vault;
// pub mod hack09_king;

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{prelude::*, utils::Anvil};

    use hack01_fallback as hack01;
    // use hack02_fallout as hack02;
    // use hack03_coin_flip as hack03;
    // use hack04_telephone as hack04;
    // use hack05_token as hack05;
    // use hack06_delegate as hack06;
    // use hack07_force as hack07;
    // use hack08_vault as hack08;
    // use hack09_king as hack09;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        let anvil = Anvil::new().port(8901u16).spawn();

        let provider = if provider.get_chainid().await.is_ok() {
            provider
        } else {
            let provider =
                Provider::<Http>::try_from(anvil.endpoint()).unwrap();

            println!(
                "Failed to connect to a running instance. Deploying levels to \
                 a new one..."
            );
            ctf::deploy(provider.clone(), "../../ctfs.json").await.unwrap();

            provider
        };

        println!("Initializing accounts...");
        let roles = ctf::Roles::new(provider)?;

        ctf::check_exploit(&roles, hack01::Exploit).await?;
        // ctf::check_exploit(&roles, hack02::Exploit).await?;
        // ctf::check_exploit(&roles, hack03::Exploit).await?;
        // ctf::check_exploit(&roles, hack04::Exploit).await?;
        // ctf::check_exploit(&roles, hack05::Exploit).await?;
        // ctf::check_exploit(&roles, hack06::Exploit).await?;
        // ctf::check_exploit(&roles, hack07::Exploit).await?;
        // ctf::check_exploit(&roles, hack08::Exploit).await?;
        // ctf::check_exploit(&roles, hack09::Exploit).await?;

        let _ = anvil.chain_id();

        Ok(())
    }
}
