use alloy::signers::local::PrivateKeySigner;
use std::error::Error;
use std::fs;

use crate::config::FILE_PATH;

pub fn run() -> Result<(), Box<dyn Error>> {
    let signer = PrivateKeySigner::random();
    let address = signer.address();
    let private_key = alloy::hex::encode(signer.to_bytes());
    fs::write(FILE_PATH, &private_key)?;
    println!("Your address and private key are generated");
    println!("Address: {}", address);
    println!("Private key: {}", private_key);
    Ok(())
}
