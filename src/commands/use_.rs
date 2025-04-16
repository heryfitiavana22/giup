use clap::Args;
use inquire::Select;

use crate::{
    git::{GitScope, set_git_email, set_git_name},
    profile_repo::get_profiles,
};

#[derive(Args, Debug)]
pub struct UseArgs {
    /// Git username
    pub username: Option<String>,

    /// Apply globally (otherwise, local by default)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub global: bool,
}

pub fn run_use(args: UseArgs) {
    let profiles = get_profiles();

    let username = match args.username {
        Some(l) => l,
        None => {
            let options: Vec<&String> = profiles.keys().collect();
            Select::new("Which profile would you like to use?", options)
                .prompt()
                .unwrap()
                .to_string()
        }
    };

    let profile = profiles.get(&username).unwrap_or_else(|| {
        eprintln!("Profile '{}' not found.", username);
        std::process::exit(1);
    });
    let scope = if args.global {
        GitScope::Global
    } else {
        GitScope::Local
    };

    set_git_name(profile.name.as_str(), scope.clone());
    set_git_email(profile.email.as_str(), scope.clone());
    println!(
        "Profile '{}' successfully activated ({}).",
        profile.name, scope
    );
}
