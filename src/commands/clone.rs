use std::process::Command;

use clap::Args;

use crate::profile_repo::get_or_select_profile_unwrap;

#[derive(Args, Debug)]
pub struct CloneArgs {
    #[arg(short, long)]
    /// Git username
    pub username: Option<String>,

    /// The git URL to clone
    pub repo_url: String,
}

pub fn run_clone(args: CloneArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to use?");

    let original_url = args.repo_url;
    let new_url = rewrite_ssh_url(&original_url, &profile.host_alias);

    println!("Cloning from: {}", new_url);

    let status = Command::new("git")
        .args(["clone", &new_url])
        .status()
        .expect("Failed to launch git");

    if status.success() {
        println!("Clone completed successfully.");
    } else {
        eprintln!("Clone failed.");
    }
}

fn rewrite_ssh_url(original: &str, alias: &str) -> String {
    if original.starts_with("git@github.com:") {
        original.replacen("git@github.com:", &format!("git@{}:", alias), 1)
    } else {
        original.to_string()
    }
}
