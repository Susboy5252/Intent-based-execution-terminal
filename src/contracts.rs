use alloy::sol;

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function deposit() external payable;
        function transfer(address to, uint256 amount) returns (bool);
        function totalSupply() view returns (uint256);
        function approve(address spender, uint256 amount) returns (bool);
        function allowance(address owner, address spender) view returns (uint256);
        function transferFrom(address from, address to, uint256 amount) returns (bool);
    }
}
