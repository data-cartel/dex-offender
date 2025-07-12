use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract AlienCodex {
        bool public contact;
        address public owner;
        bytes32[] public codex;

        function makeContact() external;
        function record(bytes32 _content) external;
        function retract() external;
        function revise(uint256 i, bytes32 _content) external;
    }
}
