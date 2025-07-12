use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Delegation {
        address public owner;
        address delegate;

        constructor(address _delegateAddress);

        fallback() external;
    }
}
