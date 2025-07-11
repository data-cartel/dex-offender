use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Privacy {
        bool public locked;
        uint256 public ID;
        bytes32[3] private data;

        constructor(bytes32[3] memory _data);
        function unlock(bytes16 _key) external;
    }
}
