use alloy::sol;

sol! {
    #[sol(rpc)]
    contract King {
        address public _king;
        uint256 public prize;
        address public owner;

        constructor() payable;

        receive() external payable;
    }
}
