use std::{fs, process::Command};

use dirs::home_dir;

use crate::profile::Profile;

pub fn generate_ssh_key(username: &str, email: &str) -> String {
    let ssh_dir = home_dir().unwrap().join(".ssh");
    let key_path = ssh_dir.join(format!("id_ed25519_{}", username));

    if key_path.exists() {
        panic!(
            "Une clé SSH avec ce nom existe déjà : {}",
            key_path.display()
        );
    }

    fs::create_dir_all(&ssh_dir).unwrap();

    let status = Command::new("ssh-keygen")
        .args([
            "-t",
            "ed25519",
            "-f",
            key_path.to_str().unwrap(),
            "-C",
            email,
        ])
        .status()
        .expect("Erreur lors de la génération de la clé SSH");

    if !status.success() {
        panic!("ssh-keygen a échoué");
    }

    key_path.to_str().unwrap().to_string()
}

pub fn update_ssh_config(profile: Profile) {
    let ssh_dir = home_dir().unwrap().join(".ssh");
    let config_path = ssh_dir.join("config");

    let identity_file = profile.ssh_key_path;
    let block = format!(
        "\nHost {alias}\n  HostName github.com\n  User git\n  IdentityFile {key}\n",
        alias = profile.host_alias,
        key = identity_file
    );

    let config_content = fs::read_to_string(&config_path).unwrap_or_default();

    if config_content.contains(&format!("Host {}", profile.host_alias)) {
        println!(
            "Le host {} existe déjà dans ~/.ssh/config",
            profile.host_alias
        );
        return;
    }

    fs::write(&config_path, format!("{config_content}{block}"))
        .expect("Impossible d'écrire ~/.ssh/config");
    println!("Entrée SSH ajoutée pour '{}'", profile.host_alias);
}

pub fn start_ssh_agent() {
    let output = Command::new("ssh-agent")
        .arg("-s")
        .output()
        .expect("Erreur lors du démarrage de ssh-agent");

    if !output.status.success() {
        panic!("ssh-agent a échoué");
    }
}
