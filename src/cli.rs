use clap::{Parser, Subcommand};

use crate::commands::{clone::CloneArgs, show::ShowArgs, use_::UseArgs};

#[derive(Parser)]
#[command(name = "gup")]
#[command(about = "Git User Profile Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add,
    Use(UseArgs),
    List,
    Show(ShowArgs),
    Clone(CloneArgs),
}
