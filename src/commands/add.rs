use inquire::{Confirm, Text};

use crate::{
    profile::Profile,
    profile_repo::{get_profile_by_username, save_profile},
    ssh::{generate_ssh_key, start_ssh_agent, update_ssh_config},
};

pub fn run_add() {
    let username = Text::new("Git username:").prompt().unwrap();
    if get_profile_by_username(&username).is_some() {
        println!("Profile with this username already exists.");
        std::process::exit(1);
    }

    let name = Text::new("Git name:").prompt().unwrap();
    let email = Text::new("Git email:").prompt().unwrap();

    let use_existing_key = Confirm::new("Use an existing SSH key?")
        .with_default(true)
        .prompt()
        .unwrap();

    let ssh_key_path = if use_existing_key {
        Text::new("Path to the existing SSH key:").prompt().unwrap()
    } else {
        generate_ssh_key(&username, &email)
    };

    let host_alias = format!("github-{}", username);

    let profile = Profile {
        name,
        email,
        ssh_key_path: ssh_key_path.clone(),
        host_alias,
        username: username.clone(),
    };

    start_ssh_agent();
    update_ssh_config(profile.clone());
    save_profile(&profile);
    println!("Profile \"{}\" added successfully.", username);
    println!("SSH key: {}", ssh_key_path);
}
