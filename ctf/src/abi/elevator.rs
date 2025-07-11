use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Elevator {
        bool public top;
        uint256 public floor;

        function goTo(uint256 _floor) external;
    }
}
