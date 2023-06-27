use ethers::providers::{Http, Provider};

use crate::level::Challenge;
use crate::roles::Roles;

pub mod lvl1_fallback;
pub mod lvl3_coin_flip;
pub mod lvl4_telephone;
pub mod lvl6_delegate;

pub use lvl1_fallback::*;
pub use lvl3_coin_flip::*;
pub use lvl4_telephone::*;
pub use lvl6_delegate::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ethernaut {
    pub level1: Level1,
    pub level3: Level3,
    pub level4: Level4,
    pub level6: Level6,
}

pub async fn set_up_ethernaut(
    provider: Provider<Http>,
) -> eyre::Result<Ethernaut> {
    let roles = Roles::new(provider)?;

    let level1 = Level1::set_up(&roles).await?;
    let level3 = Level3::set_up(&roles).await?;
    let level4 = Level4::set_up(&roles).await?;
    let level6 = Level6::set_up(&roles).await?;

    Ok(Ethernaut { level1, level3, level4, level6 })
}
