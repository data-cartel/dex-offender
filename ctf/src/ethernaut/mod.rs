use ethers::providers::{Http, Provider};

use crate::{level::Level, roles::Roles};

pub mod lvl01_fallback;
pub mod lvl02_fallout;
pub mod lvl03_coin_flip;
pub mod lvl04_telephone;
pub mod lvl05_token;
pub mod lvl06_delegate;
pub mod lvl07_force;
pub mod lvl08_vault;
pub mod lvl09_king;
pub mod lvl10_reentrancy;

use lvl01_fallback as lvl01;
use lvl02_fallout as lvl02;
use lvl03_coin_flip as lvl03;
use lvl04_telephone as lvl04;
use lvl05_token as lvl05;
use lvl06_delegate as lvl06;
use lvl07_force as lvl07;
use lvl08_vault as lvl08;
use lvl09_king as lvl09;
use lvl10_reentrancy as lvl10;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ethernaut {
    pub level01: lvl01::Target,
    pub level02: lvl02::Target,
    pub level03: lvl03::Target,
    pub level04: lvl04::Target,
    pub level05: lvl05::Target,
    pub level06: lvl06::Target,
    pub level07: lvl07::Target,
    pub level08: lvl08::Target,
    pub level09: lvl09::Target,
    pub level10: lvl10::Target,
}

pub async fn set_up_ethernaut(
    provider: Provider<Http>,
) -> eyre::Result<Ethernaut> {
    let roles = Roles::new(provider)?;

    let level01 = lvl01::Target::set_up(&roles).await?;
    let level02 = lvl02::Target::set_up(&roles).await?;
    let level03 = lvl03::Target::set_up(&roles).await?;
    let level04 = lvl04::Target::set_up(&roles).await?;
    let level05 = lvl05::Target::set_up(&roles).await?;
    let level06 = lvl06::Target::set_up(&roles).await?;
    let level07 = lvl07::Target::set_up(&roles).await?;
    let level08 = lvl08::Target::set_up(&roles).await?;
    let level09 = lvl09::Target::set_up(&roles).await?;
    let level10 = lvl10::Target::set_up(&roles).await?;

    Ok(Ethernaut {
        level01,
        level02,
        level03,
        level04,
        level05,
        level06,
        level07,
        level08,
        level09,
        level10,
    })
}
