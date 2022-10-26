use args::{Cli, Commands};
use clap::Parser;

mod args;
mod commands;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Encode(args) => commands::encode(args)?,
        Commands::Decode(args) => commands::decode(args)?,
        Commands::Remove(args) => commands::remove(args)?,
        Commands::Print(args) => commands::print_chunks(args)?,
    }

    Ok(())
}
