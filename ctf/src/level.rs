use async_trait::async_trait;

use crate::roles::Roles;

#[async_trait]
pub trait Level: Send + Sync + Clone {
    fn from_file() -> eyre::Result<Self>
    where
        Self: Sized;

    fn name(&self) -> &'static str;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn check(&self, roles: &Roles) -> eyre::Result<bool>;
}

#[async_trait]
pub trait Exploit: Send + Sync {
    type Target: Level;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &crate::Actor,
    ) -> eyre::Result<()>;
}
