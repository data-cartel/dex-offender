use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Reentrance {
        mapping(address => uint256) public balances;

        function balanceOf(address _who) external view returns (uint256 balance);
        function donate(address _to) external payable;
        function withdraw(uint256 _amount) external;

        receive() external payable;
    }
}
