use clap::Args;

use crate::{
    git::{GitScope, set_git_email, set_git_name},
    profile_repo::get_or_select_profile_unwrap,
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
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to use?");

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
