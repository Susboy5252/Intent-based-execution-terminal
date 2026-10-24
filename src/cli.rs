use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Test1")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Keygen,
    Whoami,
    SendETH {
        amount: String,
        address: String,
    },
    SendWETH {
        amount: String,
        address: String,
    },
    TransferFrom {
        from: String,
        to: String,
        amount: String,
    },
    SwapIntent {
        token_in: String,
        token_out: String,
        amount_in: String,
    },
    ApproveToken {
        address: String,
        token: String,
        amount: String,
    },
}
