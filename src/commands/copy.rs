use clap::Args;

use crate::{helper::file_to_clipboard, profile_repo::get_or_select_profile_unwrap};

#[derive(Args, Debug)]
pub struct CopyArgs {
    /// Git username to copy SSH public key
    pub username: Option<String>,
}

pub fn run_copy(args: CopyArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to copy?");
    let ssh_key_path = format!("{}.pub", profile.ssh_key_path);
    file_to_clipboard(&ssh_key_path);
    println!("Add the copied SSH public key to your GitHub account.");
}
