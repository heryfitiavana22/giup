use clap::Parser;
use cli::{Cli, Commands};
use commands::{add::run_add, clone::run_clone, list::run_list, show::run_show, use_::run_use};

pub mod cli;
pub mod commands;
pub mod file;
pub mod git;
pub mod helper;
pub mod inquire_wrapper;
pub mod profile;
pub mod profile_repo;
pub mod ssh;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => run_add(),
        Commands::List => run_list(),
        Commands::Use(args) => run_use(args),
        Commands::Show(args) => run_show(args),
        Commands::Clone(args) => run_clone(args),
    }
}
