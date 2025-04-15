use inquire::{Confirm, Text};

use crate::{
    helpers::save_profile,
    profile::Profile,
    ssh::{generate_ssh_key, start_ssh_agent, update_ssh_config},
};

pub fn run_add() {
    let username = Text::new("Nom du profil (ex: pro, perso) :")
        .prompt()
        .unwrap();

    let name = Text::new("Nom Git :").prompt().unwrap();
    let email = Text::new("Email Git :").prompt().unwrap();

    let use_existing_key = Confirm::new("Utiliser une clé SSH existante ?")
        .with_default(true)
        .prompt()
        .unwrap();

    let ssh_key_path = if use_existing_key {
        Text::new("Chemin vers la clé SSH existante :")
            .prompt()
            .unwrap()
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
    println!("Profil \"{}\" ajouté avec succès.", username);
    println!("Clé SSH : {}", ssh_key_path);
}
