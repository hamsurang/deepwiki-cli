mod cli;
mod client;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use client::DeepWikiClient;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    let client = DeepWikiClient::connect().await?;

    let output = match &cli.command {
        Command::Ask { repo, question } => client.ask_question(repo, question).await?,
        Command::Structure { repo } => client.read_wiki_structure(repo).await?,
        Command::Read { repo } => client.read_wiki_contents(repo).await?,
    };

    println!("{}", output);
    client.cancel().await?;

    Ok(())
}
