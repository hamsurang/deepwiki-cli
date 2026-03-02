mod cli;
mod client;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", std::mem::discriminant(&cli.command));
}
