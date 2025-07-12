use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Telephone {
        address public owner;

        constructor();

        function changeOwner(address _owner) public;
    }
}
