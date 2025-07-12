use alloy::sol;

sol! {
    #[sol(rpc)]
    contract CoinFlip {
        uint256 public consecutiveWins;
        uint256 lastHash;
        uint256 FACTOR;

        constructor();

        function flip(bool _guess) public returns (bool);
    }
}
