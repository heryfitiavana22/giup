use clap::Args;

use crate::{
    exit,
    file::is_file_exist,
    inquire_wrapper::{ssh_key_path_validator, text_input_with_default},
    profile::Profile,
    profile_repo::{get_or_select_profile_unwrap, get_profiles, save_profile},
    ssh::{add_to_ssh_agent, remove_ssh_key_file, start_ssh_agent, update_ssh_config},
};

#[derive(Args, Debug)]
pub struct EditArgs {
    /// Git username to edit
    pub username: Option<String>,
}

pub fn run_edit(args: EditArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to edit?");

    println!("Editing profile: {}\n", profile.username);

    let username = profile.username.clone();

    let new_name = text_input_with_default("Git name", &profile.name)
        .prompt()
        .unwrap();

    let new_email = text_input_with_default("Git email", &profile.email)
        .prompt()
        .unwrap();

    let new_key_path =
        text_input_with_default("Path to the existing SSH key:", &profile.ssh_key_path)
            .with_validator(ssh_key_path_validator)
            .prompt()
            .unwrap();

    if !is_file_exist(&new_key_path) {
        exit!(format!(
            "SSH key not found at the specified path: {}",
            new_key_path
        ));
    }

    let mut profiles = get_profiles();

    let updated_profile = Profile {
        username: username.clone(),
        name: new_name.clone(),
        email: new_email.clone(),
        ssh_key_path: new_key_path.clone(),
        host_alias: profile.host_alias,
    };

    profiles.insert(username.clone(), updated_profile.clone());

    start_ssh_agent();
    let is_key_path_changed = new_key_path != profile.ssh_key_path;
    if is_key_path_changed {
        remove_ssh_key_file(&profile.ssh_key_path);
    }
    update_ssh_config(updated_profile.clone());
    add_to_ssh_agent(&new_key_path.clone());
    save_profile(&updated_profile);
    println!("Profile '{}' updated successfully.", username);
    if is_key_path_changed {
        println!("\nNext steps:");
        println!(
            "1. Run `giup copy` and select the profile \"{}\" to copy the SSH public key.",
            username
        );
        println!("2. Add the copied SSH public key to your GitHub account.");
    }
}
