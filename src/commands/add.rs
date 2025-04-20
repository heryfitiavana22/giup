use inquire::Confirm;

use crate::{
    exit,
    file::is_file_exist,
    inquire_wrapper::{ssh_key_path_validator, text_input, text_input_with_default},
    profile::Profile,
    profile_repo::{get_profile_by_username, save_profile},
    ssh::{add_to_ssh_agent, add_to_ssh_config, generate_ssh_key, start_ssh_agent},
};

pub fn run_add() {
    let username = text_input("Git username").prompt().unwrap();
    if get_profile_by_username(&username).is_some() {
        exit!("Profile with this username already exists.");
    }

    let name = text_input_with_default("Git name", username.as_str())
        .prompt()
        .unwrap();

    let email = text_input("Git email").prompt().unwrap();

    let use_existing_key = Confirm::new("Use an existing SSH key?")
        .with_default(true)
        .prompt()
        .unwrap();

    let ssh_key_path = if use_existing_key {
        let key_path = text_input("Path to the existing SSH key:")
            .with_validator(ssh_key_path_validator)
            .prompt()
            .unwrap();

        if is_file_exist(&key_path) {
            key_path
        } else {
            exit!(format!(
                "SSH key not found at the specified path: {}",
                key_path
            ));
        }
    } else {
        generate_ssh_key(&username, &email)
    };

    let host_alias = format!("github.com-{}", username);

    let profile = Profile {
        name,
        email,
        ssh_key_path: ssh_key_path.clone(),
        host_alias,
        username: username.clone(),
    };

    start_ssh_agent();
    add_to_ssh_config(profile.clone());
    add_to_ssh_agent(&ssh_key_path.clone());
    save_profile(&profile);
    println!("Profile \"{}\" added successfully.", username);
    println!("SSH key: {}", ssh_key_path);
    println!("\nNext steps:");
    println!(
        "1. Run `gup copy` and select the profile \"{}\" to copy the SSH public key.",
        username
    );
    println!("2. Add the copied SSH public key to your GitHub account.");
}
