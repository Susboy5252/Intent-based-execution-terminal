mod cli;
mod commands;
mod config;
mod contracts;
mod permit;
mod tokens;
mod wallet;

use clap::Parser;
use dotenv::dotenv;
use std::error::Error;

use cli::{Cli, Cmd};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match &cli.cmd {
        Cmd::Keygen => commands::keygen::run()?,
        Cmd::Whoami => commands::whoami::run().await?,
        Cmd::SendETH { amount, address } => commands::send_eth::run(amount, address).await?,
        Cmd::SendWETH { amount, address } => commands::send_weth::run(amount, address).await?,
        Cmd::ApproveToken { address, token, amount } => {
            commands::approve_token::run(address, token, amount).await?
        }
        Cmd::TransferFrom { from, to, amount } => {
            commands::transfer_from::run(from, to, amount).await?
        }
        Cmd::SwapIntent { token_in, token_out, amount_in } => {
            commands::swap_intent::run(token_in, token_out, amount_in).await?
        }
    }
    Ok(())
}
