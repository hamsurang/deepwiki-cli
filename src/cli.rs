use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "deepwiki", about = "Query GitHub repo wikis via DeepWiki")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Ask an AI-powered question about a repository
    Ask {
        /// Repository name (e.g. facebook/react)
        repo: String,
        /// Question to ask
        question: String,
    },
    /// List wiki topics for a repository
    Structure {
        /// Repository name (e.g. facebook/react)
        repo: String,
    },
    /// Read full wiki contents for a repository
    Read {
        /// Repository name (e.g. facebook/react)
        repo: String,
    },
}
