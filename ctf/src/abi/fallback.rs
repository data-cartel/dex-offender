use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Fallback {
        constructor();

        mapping(address => uint256) public contributions;
        address public owner;

        function contribute() public payable;
        function getContribution() public view returns (uint256);
        function withdraw() public;

        receive() external payable;
    }
}
