mod cli;
mod config;
mod server;
mod state;

use crate::cli::Cli;
use clap::Parser;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    server::start_server::start_server(cli.config_path, cli.verbose).await
}
