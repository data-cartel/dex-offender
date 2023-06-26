///`GovernanceAction(uint128,uint64,uint64,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct GovernanceAction {
    pub value: u128,
    pub proposed_at: u64,
    pub executed_at: u64,
    pub target: ::ethers::core::types::Address,
    pub data: ::ethers::core::types::Bytes,
}
