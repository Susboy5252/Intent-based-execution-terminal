use alloy::primitives::utils::{format_ether, format_units};
use alloy::providers::Provider;
use std::error::Error;

use crate::contracts::IERC20;
use crate::tokens::{USDT, WETH};
use crate::wallet::create_signer;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let (signer, provider) = create_signer()?;
    let me = signer.address();
    let eth = provider.get_balance(me).await?;
    let weth = IERC20::new(WETH, &provider).balanceOf(me).call().await?;
    let usdt = IERC20::new(USDT, &provider).balanceOf(me).call().await?;
    println!("Address: {}", me);
    println!("Ethereum balance: {}", format_ether(eth));
    println!("WETH balance: {}", format_ether(weth));
    println!("USDT balance: {}", format_units(usdt, 6)?);
    Ok(())
}
