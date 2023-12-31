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
pub mod lvl11_elevator;
pub mod lvl12_privacy;
pub mod lvl13_gatekeeper_one;
pub mod lvl14_gatekeeper_two;
pub mod lvl15_naught_coin;
pub mod lvl16_preservation;
pub mod lvl17_recovery;
pub mod lvl18_magic_number;
pub mod lvl19_alien_codex;
pub mod lvl20_denial;
pub mod lvl21_shop;
pub mod lvl22_dex;
pub mod lvl23_dex_two;

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
use lvl11_elevator as lvl11;
use lvl12_privacy as lvl12;
use lvl13_gatekeeper_one as lvl13;
use lvl14_gatekeeper_two as lvl14;
use lvl15_naught_coin as lvl15;
use lvl16_preservation as lvl16;
use lvl17_recovery as lvl17;
use lvl18_magic_number as lvl18;
use lvl19_alien_codex as lvl19;
use lvl20_denial as lvl20;
use lvl21_shop as lvl21;
use lvl22_dex as lvl22;
use lvl23_dex_two as lvl23;

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
    pub level11: lvl11::Target,
    pub level12: lvl12::Target,
    pub level13: lvl13::Target,
    pub level14: lvl14::Target,
    pub level15: lvl15::Target,
    pub level16: lvl16::Target,
    pub level17: lvl17::Target,
    pub level18: lvl18::Target,
    pub level19: lvl19::Target,
    pub level20: lvl20::Target,
    pub level21: lvl21::Target,
    pub level22: lvl22::Target,
    pub level23: lvl23::Target,
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
    let level11 = lvl11::Target::set_up(&roles).await?;
    let level12 = lvl12::Target::set_up(&roles).await?;
    let level13 = lvl13::Target::set_up(&roles).await?;
    let level14 = lvl14::Target::set_up(&roles).await?;
    let level15 = lvl15::Target::set_up(&roles).await?;
    let level16 = lvl16::Target::set_up(&roles).await?;
    let level17 = lvl17::Target::set_up(&roles).await?;
    let level18 = lvl18::Target::set_up(&roles).await?;
    let level19 = lvl19::Target::set_up(&roles).await?;
    let level20 = lvl20::Target::set_up(&roles).await?;
    let level21 = lvl21::Target::set_up(&roles).await?;
    let level22 = lvl22::Target::set_up(&roles).await?;
    let level23 = lvl23::Target::set_up(&roles).await?;

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
        level11,
        level12,
        level13,
        level14,
        level15,
        level16,
        level17,
        level18,
        level19,
        level20,
        level21,
        level22,
        level23,
    })
}
