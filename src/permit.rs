use alloy::sol;

sol! {
    #[derive(serde::Deserialize)]
    struct PermitDetails {
        address token;
        uint160 amount;
        uint48 expiration;
        uint48 nonce;
    }
    #[derive(serde::Deserialize)]
    struct PermitSingle {
        PermitDetails details;
        address spender;
        uint256 sigDeadline;
    }
}
