use alloy::sol;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Dex {
        address public token1;
        address public token2;
        address public owner;

        constructor();
        function addLiquidity(address token_address, uint256 amount) external;
        function approve(address spender, uint256 amount) external;
        function balanceOf(address token, address account) external view returns (uint256);
        function getSwapPrice(address from, address to, uint256 amount) external view returns (uint256);
        function setTokens(address _token1, address _token2) external;
        function swap(address from, address to, uint256 amount) external;
    }
}
