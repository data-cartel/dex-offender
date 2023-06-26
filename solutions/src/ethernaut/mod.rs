pub mod lvl1_fallback;
<<<<<<< HEAD
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
=======
// pub mod lvl3_coin_flip;     <===== OPA)))
// pub mod lvl4_telephone;     <===== OPA)))
// pub mod lvl6_delegate;     <===== OPA)))
>>>>>>> c32e0e4 (lol)))
