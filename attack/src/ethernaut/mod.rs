pub mod hack01_fallback;

#[cfg(test)]
mod tests {
    use super::*;

    use hack01_fallback as hack01;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        println!("Initializing accounts...");
        let roles = ctf::Roles::new("http://127.0.0.1:8545").await?;

        ctf::check_exploit(&roles, hack01::Exploit).await?;

        Ok(())
    }
}
