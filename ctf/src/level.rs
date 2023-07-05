use crate::roles::*;
use async_trait::async_trait;

#[async_trait]
pub trait Level {
    fn name(&self) -> &'static str;

    fn from_file() -> eyre::Result<Self>
    where
        Self: Sized;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn check(&self, roles: &Roles) -> eyre::Result<bool>;
}

#[async_trait]
pub trait Exploit {
    type Target: Level;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()>;
}
