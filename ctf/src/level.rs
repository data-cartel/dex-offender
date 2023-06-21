use crate::roles::*;
use async_trait::async_trait;

#[async_trait]
pub trait Challenge {
    const DESCRIPTION: &'static str;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn check(self, roles: Roles) -> eyre::Result<Self>
    where
        Self: Sized;
}

#[async_trait]
pub trait Solution {
    type Level: Challenge;

    async fn solve(
        self,
        challenge: &Self::Level,
        offender: Actor,
    ) -> eyre::Result<()>;
}