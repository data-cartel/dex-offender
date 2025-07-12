use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract MagicNum {
        address public solver;

        function setSolver(address _solver) external;
    }
}
