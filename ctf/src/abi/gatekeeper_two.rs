use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract GatekeeperTwo {
        address public entrant;

        function enter(bytes8 _gateKey) external returns (bool);
    }
}
