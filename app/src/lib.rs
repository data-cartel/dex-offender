use ethers::prelude::*;

pub fn to_scaled_u256<U>(amount: U) -> U256
where
    U: Into<U256>,
{
    amount.into() * U256::from(10).pow(U256::from(18))
}
