use clap::Parser;
use cli::{Cli, Commands};

pub mod cli;
pub mod commands;
pub mod profile_repo;
pub mod ssh;
pub mod profile;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => commands::add::run_add(),
    }
}
