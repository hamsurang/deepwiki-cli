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
    let (repo, query_type) = repo_and_query_type(&cli.command);

    if let Ok(mock_text) = std::env::var("DEEPWIKI_CLI_MOCK_TEXT") {
        println!(
            "{}",
            output::format_for_claude(&mock_text, repo, query_type)
        );
        return Ok(());
    }

    let client = DeepWikiClient::connect().await?;

    let text = match &cli.command {
        Command::Ask { repo, question } => client.ask_question(repo, question).await?,
        Command::Structure { repo } => client.read_wiki_structure(repo).await?,
        Command::Read { repo } => client.read_wiki_contents(repo).await?,
    };

    println!("{}", output::format_for_claude(&text, repo, query_type));
    client.cancel().await?;

    Ok(())
}

fn repo_and_query_type(command: &Command) -> (&str, &str) {
    match command {
        Command::Ask { repo, .. } => (repo, "ask"),
        Command::Structure { repo } => (repo, "structure"),
        Command::Read { repo } => (repo, "read"),
    }
}
