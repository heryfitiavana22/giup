use clap::Args;
use inquire::Confirm;

use crate::{
    exit,
    profile_repo::{get_or_select_profile_unwrap, remove_profile},
    ssh::{remove_in_ssh_config, remove_ssh_key_file},
};

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Git username to remove
    pub username: Option<String>,
}

pub fn run_remove(args: RemoveArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to remove?");

    let is_remove = Confirm::new(&format!("Removing profile: {} ?", profile.username))
        .with_default(true)
        .prompt()
        .unwrap();

    if !is_remove {
        exit!("Profile removal cancelled.");
    }

    remove_in_ssh_config(profile.clone());
    remove_profile(&profile);
    remove_ssh_key_file(&profile.ssh_key_path);
    remove_ssh_key_file(&format!("{}.pub", profile.ssh_key_path));
    println!("Profile \"{}\" removed successfully.", profile.username);
}
