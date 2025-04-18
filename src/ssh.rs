use std::{fs, path::PathBuf, process::Command};

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

pub fn get_ssh_config() -> PathBuf {
    let ssh_dir = home_dir().unwrap().join(".ssh");
    let config_path = ssh_dir.join("config");

    if !config_path.exists() {
        fs::File::create(&config_path).expect("Unable to create ~/.ssh/config");
    }

    config_path
}

pub fn add_to_ssh_config(profile: Profile) {
    let config_content = read_ssh_config();

    if config_content.contains(&format!("Host {}", profile.host_alias)) {
        println!(
            "The host {} already exists in ~/.ssh/config",
            profile.host_alias
        );
        return;
    }

    let block = generate_ssh_content(&profile);

    write_ssh_config(format!("{config_content}{block}").as_str());
    println!("SSH entry added for '{}'", profile.host_alias);
}

pub fn update_ssh_config(profile: Profile) {
    let config_content = read_ssh_config();

    if !config_content.contains(&format!("Host {}", profile.host_alias)) {
        println!(
            "The host {} doesn't exists in ~/.ssh/config",
            profile.host_alias
        );
        return;
    }

    let block = generate_ssh_content(&profile);
    let cleaned_config = remove_host_block(&config_content, &profile.host_alias);

    write_ssh_config(format!("{cleaned_config}{block}").as_str());
    println!("SSH entry updated for '{}'", profile.host_alias);
}

pub fn remove_in_ssh_config(profile: Profile) {
    let config_content = read_ssh_config();

    if !config_content.contains(&format!("Host {}", profile.host_alias)) {
        println!(
            "The host {} doesn't exists in ~/.ssh/config",
            profile.host_alias
        );
        return;
    }

    let cleaned_config = remove_host_block(&config_content, &profile.host_alias);

    write_ssh_config(format!("{cleaned_config}").as_str());
    println!("SSH entry deleted for '{}'", profile.host_alias);
}

pub fn remove_ssh_key_file(key_path: &str) {
    if let Err(e) = fs::remove_file(key_path) {
        exit!(format!("Failed to remove the SSH key: {}", e));
    }

    println!("SSH key file removed successfully");
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

pub fn test_ssh_host(host_alias: &str) {
    let output = Command::new("ssh")
        .args(["-T", format!("git@{}", host_alias).as_str()])
        .output()
        .expect("Error while testing SSH connection");

    let output_str = String::from_utf8_lossy(&output.stderr);
    println!("{}", output_str);
}

pub fn check_ssh_keygen_available() {
    let result = Command::new("ssh-keygen").arg("-V").output();

    if result.is_err() {
        exit!("Command not found: 'ssh-keygen'");
    }
}

fn remove_host_block(config: &str, host_to_remove: &str) -> String {
    let mut result = String::new();
    let mut lines = config.lines();
    let mut skipping = false;

    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("Host ") {
            if line.trim_start() == format!("Host {}", host_to_remove) {
                skipping = true;
                continue;
            } else {
                skipping = false;
            }
        }

        if skipping {
            if line.starts_with(' ') || line.starts_with('\t') {
                continue;
            } else {
                skipping = false;
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

fn generate_ssh_content(profile: &Profile) -> String {
    format!(
        "\nHost {alias}\n  HostName github.com\n  User git\n  IdentityFile {key}\n  IdentitiesOnly yes",
        alias = profile.host_alias,
        key = profile.ssh_key_path
    )
}

fn write_ssh_config(config: &str) {
    let config_path = get_ssh_config();
    fs::write(&config_path, config).expect("Unable to write to ~/.ssh/config");
}

fn read_ssh_config() -> String {
    let config_path = get_ssh_config();
    let config_content = read_file_to_string(&config_path);

    config_content
}
