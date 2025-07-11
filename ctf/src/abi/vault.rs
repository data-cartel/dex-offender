use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Vault {
        bool public locked;
        bytes32 private password;

        constructor(bytes32 _password);

        function unlock(bytes32 _password) public;
    }
}
