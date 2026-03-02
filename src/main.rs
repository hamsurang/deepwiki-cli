mod cli;
mod client;
mod output;

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

    let (text, query_type) = match &cli.command {
        Command::Ask { repo, question } => (client.ask_question(repo, question).await?, "ask"),
        Command::Structure { repo } => (client.read_wiki_structure(repo).await?, "structure"),
        Command::Read { repo } => (client.read_wiki_contents(repo).await?, "read"),
    };

    let repo = match &cli.command {
        Command::Ask { repo, .. } => repo,
        Command::Structure { repo } => repo,
        Command::Read { repo } => repo,
    };

    println!("{}", output::format_for_claude(&text, repo, query_type));
    client.cancel().await?;

    Ok(())
}
