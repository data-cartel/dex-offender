pub mod lvl1_fallback;
pub mod lvl3_coin_flip;
pub mod lvl4_telephone;
pub mod lvl6_delegate;

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::prelude::*;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        println!("Initializing accounts...");
        let roles = ctf::Roles::new(provider)?;

        ctf::check_solution(&roles, lvl1_fallback::Solution).await?;
        ctf::check_solution(&roles, lvl3_coin_flip::Solution).await?;
        ctf::check_solution(&roles, lvl4_telephone::Solution).await?;
        ctf::check_solution(&roles, lvl6_delegate::Solution).await?;

        Ok(())
    }
}
