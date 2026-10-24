use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use std::error::Error;
use std::fs;

use crate::config::{rpc_url, FILE_PATH};

pub fn create_signer() -> Result<(PrivateKeySigner, impl Provider), Box<dyn Error>> {
    match fs::read_to_string(FILE_PATH) {
        Ok(saved_key) => {
            let signer: PrivateKeySigner = saved_key.trim().parse()?;
            let provider = ProviderBuilder::new()
                .wallet(signer.clone())
                .connect_http(rpc_url().parse()?);
            Ok((signer, provider))
        }
        Err(e) => {
            println!("No wallet found! Run 'cargo run -- keygen' first.");
            Err(e.into())
        }
    }
}
