use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;

pub use crate::abi::{dex_two::DexTwo, swappable_token_two::SwappableTokenTwo};
use crate::{roles::*, Level};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level23)
    }

    fn name(&self) -> &'static str { "DexTwo" }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender, some_user: _ } = roles;

        println!("Deploying the DexTwo contract...");
        let contract = DexTwo::deploy(deployer, ()).await?;
        let token1 = SwappableTokenTwo::deploy(
            deployer,
            (
                *contract.address(),
                String::from("Token 1"),
                String::from("TKN1"),
                U256::from(110),
            ),
        ).await?;
        let token2 = SwappableTokenTwo::deploy(
            deployer,
            (
                *contract.address(),
                String::from("Token 2"),
                String::from("TKN2"),
                U256::from(110),
            ),
        ).await?;

        let pending = contract.set_tokens(*token1.address(), *token2.address()).send().await?;
        let _receipt = pending.get_receipt().await?;

        let pending = token1.approve(*contract.address(), U256::from(100)).send().await?;
        let _receipt = pending.get_receipt().await?;

        let pending = token2.approve(*contract.address(), U256::from(100)).send().await?;
        let _receipt = pending.get_receipt().await?;

        let pending = contract
            .add_liquidity(*token1.address(), U256::from(100))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let pending = contract
            .add_liquidity(*token2.address(), U256::from(100))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let pending = token1
            .transfer(offender.default_signer_address(), U256::from(10))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let pending = token2
            .transfer(offender.default_signer_address(), U256::from(10))
            .send()
            .await?;
        let _receipt = pending.get_receipt().await?;

        let target = Target { address: *contract.address() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let Roles { deployer, offender: _, some_user: _ } = roles;
        let contract = DexTwo::new(self.address, deployer);
        println!("Checking that you have stolen all tokens of both types...");

        let token1 = SwappableTokenTwo::new(
            contract.token_1().call().await?._0,
            deployer,
        );
        let token2 = SwappableTokenTwo::new(
            contract.token_2().call().await?._0,
            deployer,
        );

        Ok(token1.balance_of(*contract.address()).call().await?._0 == U256::from(0)
            && token2.balance_of(*contract.address()).call().await?._0 == U256::from(0))
    }
}
