use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Delegate {
        address public owner;

        constructor(address _owner);

        function pwn() public;
    }
}
