use alloy::primitives::utils::{format_ether, parse_ether};
use alloy::primitives::Address;
use std::error::Error;

use crate::contracts::IERC20;
use crate::tokens::WETH;
use crate::wallet::create_signer;

pub async fn run(from: &str, to: &str, amount: &str) -> Result<(), Box<dyn Error>> {
    let (_, provider) = create_signer()?;
    let recipient: Address = to.parse()?;
    let sender: Address = from.parse()?;
    let value = parse_ether(amount)?;
    let pending = IERC20::new(WETH, provider)
        .transferFrom(sender, recipient, value)
        .send()
        .await?;
    println!("Transaction hash: {}", pending.tx_hash());
    let receipt = pending.get_receipt().await?;
    println!(
        "Confirmed in block: {}",
        receipt.block_number.unwrap_or_default()
    );
    println!(
        "Trasnfered\nfrom {}\nto {}\namount of WETH: {}",
        sender,
        recipient,
        format_ether(value)
    );
    Ok(())
}
