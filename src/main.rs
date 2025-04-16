use clap::Parser;
use cli::{Cli, Commands};
use commands::{add::run_add, use_::run_use};

pub mod cli;
pub mod commands;
pub mod profile;
pub mod profile_repo;
pub mod ssh;
pub mod git;
pub mod inquire_wrapper;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => run_add(),
        Commands::Use(args) => run_use(args),
    }
}
