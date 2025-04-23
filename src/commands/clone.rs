use std::process::Command;

use clap::Args;

use crate::{
    commands::use_::{UseArgs, run_use},
    profile_repo::get_or_select_profile_unwrap,
};

#[derive(Args, Debug)]
pub struct CloneArgs {
    #[arg(short, long)]
    /// Git username to use
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
        if let Some(repo_name_unsafe) = original_url.split('/').next_back() {
            let repo_name = repo_name_unsafe.replace(".git", "");

            std::env::set_current_dir(repo_name)
                .expect("Failed to change directory to the cloned repository");
            
            run_use(UseArgs {
                username: Some(profile.username.clone()),
                global: false,
            });
            println!("Clone completed successfully.");
        }
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
