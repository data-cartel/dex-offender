use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Denial {
        address public owner;
        address public partner;

        function contractBalance() external view returns (uint256);
        function setWithdrawPartner(address _partner) external;
        function withdraw() external;

        receive() external payable;
    }
}
