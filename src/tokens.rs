use alloy::primitives::{address, Address};

pub const ETH: Address = address!("0000000000000000000000000000000000000000");
pub const WETH: Address = address!("fFf9976782d46CC05630D1f6eBAb18b2324d6B14");
pub const USDT: Address = address!("7169D38820dfd117C3FA1f22a697dBA58d90BA06");
pub const USDC: Address = address!("776b6fC2eD15D6Bb5Fc32e0c89DE68683118c62A");

pub fn parse_token(symbol: &str) -> (Address, u8) {
    match symbol.to_lowercase().as_str() {
        "eth" => (ETH, 18),
        "weth" => (WETH, 18),
        "usdt" => (USDT, 6),
        "usdc" => (USDC, 6),
        other => panic!("Unkown token symbol: {other}"),
    }
}
