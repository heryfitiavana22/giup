use clap::{Parser, Subcommand};

use crate::commands::{
    clone::CloneArgs, edit::EditArgs, remove::RemoveArgs, show::ShowArgs, test::TestArgs,
    use_::UseArgs,
};

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
    List,
    Current,
    Use(UseArgs),
    Show(ShowArgs),
    Clone(CloneArgs),
    Test(TestArgs),
    Edit(EditArgs),
    Remove(RemoveArgs),
}
