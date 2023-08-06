pub mod hack01_fallback;

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::prelude::*;

    use hack01_fallback as hack01;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;

        println!("Initializing accounts...");
        let roles = ctf::Roles::new(provider)?;

        ctf::check_exploit(&roles, hack01::Exploit).await?;

        Ok(())
    }
}
