use std::{fs, process::Command};

use dirs::home_dir;

use crate::{exit, file::read_file_to_string, profile::Profile};

pub fn generate_ssh_key(username: &str, email: &str) -> String {
    check_ssh_keygen_available();
    let ssh_dir = home_dir().unwrap().join(".ssh");
    let key_path = ssh_dir.join(format!("id_ed25519_{}", username));

    if key_path.exists() {
        exit!(format!(
            "An SSH key with this name already exists: {}",
            key_path.display()
        ));
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
        .expect("Error while generating the SSH key");

    if !status.success() {
        exit!("ssh-keygen failed");
    }

    key_path.to_str().unwrap().to_string()
}

pub fn check_ssh_keygen_available() {
    let result = Command::new("ssh-keygen").arg("-V").output();

    if result.is_err() {
        exit!("Command not found: 'ssh-keygen'");
    }
}

pub fn update_ssh_config(profile: Profile) {
    let ssh_dir = home_dir().unwrap().join(".ssh");
    let config_path = ssh_dir.join("config");

    let identity_file = profile.ssh_key_path;
    let block = format!(
        "\nHost {alias}\n  HostName github.com\n  User git\n  IdentityFile {key}\n  IdentitiesOnly yes",
        alias = profile.host_alias,
        key = identity_file
    );

    let config_content = read_file_to_string(&config_path);

    if config_content.contains(&format!("Host {}", profile.host_alias)) {
        println!(
            "The host {} already exists in ~/.ssh/config",
            profile.host_alias
        );
        return;
    }

    fs::write(&config_path, format!("{config_content}{block}"))
        .expect("Unable to write to ~/.ssh/config");
    println!("SSH entry added for '{}'", profile.host_alias);
}

pub fn start_ssh_agent() {
    let output = Command::new("ssh-agent")
        .arg("-s")
        .output()
        .expect("Error while starting ssh-agent");

    if !output.status.success() {
        exit!("ssh-agent failed");
    }
}

pub fn add_to_ssh_agent(key_path: &str) {
    let output = Command::new("ssh-add")
        .arg(key_path)
        .output()
        .expect("Error while adding SSH key to agent");

    if !output.status.success() {
        exit!("ssh-add failed");
    }
    println!("SSH key added to agent");
}
