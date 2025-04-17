use clap::Parser;
use cli::{Cli, Commands};
use commands::{
    add::run_add, clone::run_clone, edit::run_edit, list::run_list, remove::run_remove,
    show::run_show, test::run_test, use_::run_use,
};

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
        Commands::Test(args) => run_test(args),
        Commands::Edit(args) => run_edit(args),
        Commands::Remove(args) => run_remove(args),
    }
}
