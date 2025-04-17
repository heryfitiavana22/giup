use clap::Args;

use crate::{
    exit,
    file::is_file_exist,
    inquire_wrapper::text_input_with_default,
    profile::Profile,
    profile_repo::{get_or_select_profile_unwrap, get_profiles, save_profile},
    ssh::{add_to_ssh_agent, start_ssh_agent, update_ssh_config},
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
    update_ssh_config(updated_profile.clone());
    add_to_ssh_agent(&new_key_path.clone());
    // TODO: copy the pub key to the clipboard
    save_profile(&updated_profile);
    println!("Profile '{}' updated successfully.", username);
}
