use alloy::network::TransactionBuilder;
use alloy::primitives::utils::{format_ether, parse_ether};
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use std::error::Error;

use crate::wallet::create_signer;

pub async fn run(amount: &str, address: &str) -> Result<(), Box<dyn Error>> {
    let (signer, provider) = create_signer()?;
    let recipient: Address = address.parse()?;
    let value = parse_ether(amount)?;
    let me = signer.address();
    let eth = provider.get_balance(me).await?;
    println!("Eth balance before transfer: {}", format_ether(eth));
    let tx = TransactionRequest::default()
        .with_to(recipient)
        .with_value(value);
    let pending = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending.tx_hash());
    let receipt = pending.get_receipt().await?;
    println!(
        "Successfully mined in block: {}",
        receipt.block_number.unwrap_or_default()
    );
    Ok(())
}
