use clap::Args;

use crate::profile_repo::get_or_select_profile_unwrap;

#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Git username to display
    pub username: Option<String>,
}

pub fn run_show(args: ShowArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to show?");

    println!("");
    println!("Username : {}", profile.username);
    println!("Name     : {}", profile.name);
    println!("Email    : {}", profile.email);
    println!("SSH Key  : {}", profile.ssh_key_path);
    println!("Host     : {}", profile.host_alias);
}
