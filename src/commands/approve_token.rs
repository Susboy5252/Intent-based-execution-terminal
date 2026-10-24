use alloy::primitives::utils::parse_units;
use alloy::primitives::Address;
use std::error::Error;

use crate::contracts::IERC20;
use crate::tokens::parse_token;
use crate::wallet::create_signer;

pub async fn run(address: &str, token: &str, amount: &str) -> Result<(), Box<dyn Error>> {
    let (_, provider) = create_signer()?;
    let (parsed_token, units) = parse_token(token);
    let recipient: Address = address.parse()?;
    let value = parse_units(amount, units)?.into();
    let pending = IERC20::new(parsed_token, provider)
        .approve(recipient, value)
        .send()
        .await?;
    println!("Transaction hash: {}", pending.tx_hash());
    let receipt = pending.get_receipt().await?;
    println!(
        "Confirmed in block: {}",
        receipt.block_number.unwrap_or_default()
    );
    println!(
        "You allowed {}, to use your {} amount of {}",
        recipient, amount, token
    );
    Ok(())
}
