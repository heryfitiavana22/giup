use clap::{Parser, Subcommand};

use crate::commands::{
    clone::CloneArgs, copy::CopyArgs, edit::EditArgs, remove::RemoveArgs, show::ShowArgs,
    test::TestArgs, use_::UseArgs,
};

#[derive(Parser)]
#[command(name = "gup")]
#[command(about = "Git User Profile Manager", long_about = None)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new Git user profile
    Add,

    /// List all available profiles
    List,

    /// Show the currently active Git profile (local or global)
    Current,

    /// Set the current Git user profile (local or global)
    Use(UseArgs),

    /// Show details of a specific profile
    Show(ShowArgs),

    /// Clone a Git repo using a specific profile
    Clone(CloneArgs),

    /// Test if a profile is correctly configured (SSH)
    Test(TestArgs),

    /// Edit an existing profile
    Edit(EditArgs),

    /// Remove a saved profile
    Remove(RemoveArgs),

    /// Copy the SSH public key of a profile to the clipboard
    Copy(CopyArgs),
}
