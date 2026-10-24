use alloy::dyn_abi::Eip712Domain;
use alloy::hex;
use alloy::network::TransactionBuilder;
use alloy::primitives::utils::parse_units;
use alloy::primitives::{Address, Bytes};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::Signer;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::error::Error;
use std::str::FromStr;
//Test
use crate::config::{rpc_url, SEPOLIA_CHAIN_ID};
use crate::permit::PermitSingle;
use crate::tokens::parse_token;
use crate::wallet::create_signer;

pub async fn run(
    token_in: &str,
    token_out: &str,
    amount_in: &str,
) -> Result<(), Box<dyn Error>> {
    let (signer, provider) = create_signer()?;
    let client = Client::new();
    let (parsed_token_in, decimals_in) = parse_token(token_in);
    let (parsed_token_out, _) = parse_token(token_out);

    let check_approval = json!({
        "walletAddress": &signer.address(),
        "amount": parse_units(amount_in, decimals_in)?.to_string(),
        "token": parsed_token_in.to_string(),
        "chainId": SEPOLIA_CHAIN_ID,
        "tokenOut": parsed_token_out.to_string(),
        "tokenOutChainId": SEPOLIA_CHAIN_ID
    });

    let response_approval = client
        .post("https://trade-api.gateway.uniswap.org/v1/check_approval")
        .header("x-api-key", env::var("API_KEY")?)
        .header("Content-Type", "application/json")
        .json(&check_approval)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(data_approval) = response_approval.get("approval").and_then(|v| v.as_object()) {
        let data = &data_approval["data"].as_str().ok_or("Missing approval data")?;
        dbg!(data_approval);
        let calldata = alloy::primitives::Bytes::from_str(data)?;
        let value = alloy::primitives::U256::ZERO;
        let tx = TransactionRequest::default()
            .with_to(parsed_token_in)
            .with_input(calldata)
            .with_value(value);
        let pending = provider.send_transaction(tx).await?;
        println!("Approving token! Transaction Hash: {}", pending.tx_hash());
        let receipt = pending.get_receipt().await?;
        println!(
            "Approve mined in block: {}",
            receipt.block_number.unwrap_or_default()
        );
    }

    //let value = parse_units(amount_in, decimals_in).unwrap().into();
    let payload = json!({
        "tokenIn": parsed_token_in.to_string(), // ETH
        "tokenOut": parsed_token_out.to_string(), // USDT
        "tokenInChainId": SEPOLIA_CHAIN_ID, // Sepolia chain ID
        "tokenOutChainId": SEPOLIA_CHAIN_ID,
        "type": "EXACT_INPUT",
        "amount": parse_units(amount_in, decimals_in)?.to_string(),
        "swapper": signer.address().to_string(),
        "routingPreference": "BEST_PRICE",
        "slippageTolerance": 0.5,
        //"protocols": ["UNISWAPX_LATEST"]
    });

    let response = client
        .post("https://trade-api.gateway.uniswap.org/v1/quote")
        .header("x-api-key", env::var("API_KEY")?)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!(
        "Received quote, permit data and routing from Uniswap API {:#?}",
        response
    );

    let routing = response["routing"].to_string();

    if let Some(permit_data) = response.get("permitData").and_then(|v| v.as_object()) {
        let domain_json = &permit_data["domain"];
        let domain: Eip712Domain = serde_json::from_value(domain_json.clone())?;
        let values_json = &permit_data["values"];
        let values: PermitSingle = serde_json::from_value(values_json.clone())?;
        println!("Permita data received! Signing intent...");
        let signature = signer.sign_typed_data(&values, &domain).await?;

        if routing == "CLASSIC" || routing == "WRAP" || routing == "UNWRAP" || routing == "BRIDGE" {
            let order_payload = json!({
                "signature": format!("0x{}", hex::encode(signature.as_bytes())),
                "quote": response["quote"],
                "permitData": permit_data
            });

            let order_response = client
                .post("https://trade-api.gateway.uniswap.org/v1/swap")
                .header("x-api-key", env::var("API_KEY")?)
                .header("Content-Type", "application/json")
                .json(&order_payload)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            println!("Order successfully broadcasted to Uniswap Solvers!");
            println!("{:#?}", order_response);

            if let Some(swap) = order_response.get("swap") {
                let to_address_json = &swap["to"];
                let to_address: Address = serde_json::from_value(to_address_json.clone())?;
                let calldata = Bytes::from_str(order_response["swap"]["data"].as_str().unwrap())?;
                let hex_value = order_response["swap"]["value"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x");
                let value = if hex_value.is_empty() {
                    alloy::primitives::U256::ZERO
                } else {
                    alloy::primitives::U256::from_str_radix(hex_value, 16)?
                };

                let tx = TransactionRequest::default()
                    .with_to(to_address)
                    .with_input(calldata)
                    .with_value(value);
                let pending = provider.send_transaction(tx).await?;
                println!("Swap broadcasted! Transaction Hash: {}", pending.tx_hash());
                let receipt = pending.get_receipt().await?;
                println!(
                    "Swap mined in block: {}",
                    receipt.block_number.unwrap_or_default()
                );
            }
        } else if routing == "DUTCH_V2" || routing == "DUTCH_V3" || routing == "PRIORITY" {
            let order_payload = json!({
                "signature": format!("0x{}", hex::encode(signature.as_bytes())),
                "quote": response["quote"],
            });

            let order_response = client
                .post("https://trade-api.gateway.uniswap.org/v1/order")
                .header("x-api-key", env::var("API_KEY")?)
                .header("Content-Type", "application/json")
                .json(&order_payload)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            println!("Order successfully broadcasted to Uniswap Solvers!");
            println!("{:#?}", order_response);

            //"Add implementation to this block and to the swapTransaction block, understand what are these routings"
            todo!("Add implementation to this block and to the swapTransaction block, understand what are these routings");
        }
    } else if let Some(swap_tx) = response.get("swapTransaction") {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect_http(rpc_url().parse()?);
        let to_address_json = &swap_tx["to"];
        let to_address: Address = serde_json::from_value(to_address_json.clone())?;
        let calldata = Bytes::from_str(swap_tx["data"].as_str().unwrap())?;
        let hex_value = swap_tx["value"].as_str().unwrap().trim_start_matches("0x");
        let value = if hex_value.is_empty() {
            alloy::primitives::U256::ZERO
        } else {
            alloy::primitives::U256::from_str_radix(hex_value, 16)?
        };
        let tx = TransactionRequest::default()
            .with_to(to_address)
            .with_input(calldata)
            .with_value(value);
        let pending = provider.send_transaction(tx).await?;
        println!("Swap broadcasted! Transaction Hash: {}", pending.tx_hash());
        let receipt = pending.get_receipt().await?;
        println!(
            "Swap mined in block: {}",
            receipt.block_number.unwrap_or_default()
        );
    }

    Ok(())
}
