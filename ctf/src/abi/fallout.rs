use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Fallout {
        mapping(address => uint256) allocatorBalance;
        address payable public owner;

        function Fal1out() public payable;
        function allocate() public payable;
        function collectAllocations() public;
        function sendAllocation(address payable allocator) public;
    }
}
