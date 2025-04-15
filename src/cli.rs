use clap::{Parser, Subcommand};

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
}
