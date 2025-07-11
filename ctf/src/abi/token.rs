use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Token {
        uint256 public totalSupply;
        mapping(address => uint256) balances;

        constructor(uint256 _initialSupply);

        function balanceOf(address _owner) public view returns (uint256 balance);
        function transfer(address _to, uint256 _value) public returns (bool);
    }
}
