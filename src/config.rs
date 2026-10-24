pub const FILE_PATH: &str = ".wallet";
pub const SEPOLIA_CHAIN_ID: u64 = 11_155_111;

pub fn rpc_url() -> String {
    std::env::var("RPC_URL")
        .unwrap_or_else(|_| "https://ethereum-sepolia-rpc.publicnode.com".to_string())
}
