use std::{collections::HashMap, fs, path::PathBuf};

use dirs::config_dir;
use inquire::Select;

use crate::{
    exit,
    file::{read_file_to_string, write_file},
    profile::Profile,
};

pub fn get_config_path() -> PathBuf {
    let mut config_path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("gup");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("profiles.toml");

    config_path
}

pub fn get_profile_by_username(username: &str) -> Option<Profile> {
    let profiles = get_profiles();
    profiles.get(username).cloned()
}

pub fn get_profile_by_username_unwrap(username: &str) -> Profile {
    get_profile_by_username(username).unwrap_or_else(|| {
        exit!(format!("Profile '{}' not found.", username));
    })
}

pub fn get_profiles() -> HashMap<String, Profile> {
    let config_path = get_config_path();
    if config_path.exists() {
        let content = read_file_to_string(&config_path);
        toml::from_str::<toml::Value>(&content)
            .ok()
            .and_then(|val| val.as_table().cloned())
            .unwrap_or_else(|| toml::map::Map::new())
            .iter()
            .filter_map(|(k, v)| {
                if let Ok(profile) = Profile::try_from(v.clone()) {
                    Some((k.clone(), profile))
                } else {
                    None
                }
            })
            .collect::<std::collections::HashMap<String, Profile>>()
    } else {
        HashMap::new()
    }
}

pub fn get_or_select_profile(username: Option<String>, message: &str) -> Option<Profile> {
    let profiles = get_profiles();

    let username = match username {
        Some(l) => l,
        None => {
            let options: Vec<&String> = profiles.keys().collect();
            Select::new(&message, options).prompt().unwrap().to_string()
        }
    };

    profiles.get(&username).cloned()
}

pub fn get_or_select_profile_unwrap(username: Option<String>, message: &str) -> Profile {
    get_or_select_profile(username.clone(), message).unwrap_or_else(|| {
        exit!(format!(
            "Profile '{}' not found.",
            username.unwrap_or_default()
        ));
    })
}

pub fn save_profile(profile: &Profile) {
    let mut config_path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("gup");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("profiles.toml");

    let mut profiles = get_profiles();
    profiles.insert(profile.username.clone(), profile.clone());

    let toml_profiles = profiles
        .into_iter()
        .map(|(k, v)| (k, toml::Value::try_from(v).unwrap()))
        .collect::<toml::map::Map<String, toml::Value>>();

    let serialized = toml::to_string_pretty(&toml::Value::Table(toml_profiles)).unwrap();
    write_file(config_path, serialized);
}
