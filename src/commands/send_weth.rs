use alloy::primitives::utils::parse_ether;
use alloy::primitives::Address;
use std::error::Error;

use crate::contracts::IERC20;
use crate::tokens::WETH;
use crate::wallet::create_signer;

pub async fn run(amount: &str, address: &str) -> Result<(), Box<dyn Error>> {
    let (_, provider) = create_signer()?;
    let value = parse_ether(amount)?;
    let recipient: Address = address.parse()?;
    let weth = IERC20::new(WETH, provider);
    let pending = weth.transfer(recipient, value).send().await?;
    println!("send tx: {}", pending.tx_hash());
    let receipt = pending.get_receipt().await?;
    println!(
        "mined in block {:?}, success: {}",
        receipt.block_number,
        receipt.status()
    );
    Ok(())
}
