use std::{fs, path::PathBuf};

use dirs::config_dir;

use crate::profile::Profile;


pub fn save_profile(profile: &Profile) {
    let mut config_path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("gup");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("profiles.toml");

    let mut profiles = if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap();
        toml::from_str::<toml::Value>(&content)
            .ok()
            .and_then(|val| val.as_table().cloned())
            .unwrap_or_else(|| toml::map::Map::new())
    } else {
        toml::map::Map::new()
    };

    profiles.insert(
        profile.username.clone(),
        toml::Value::try_from(profile).unwrap(),
    );

    let serialized = toml::to_string_pretty(&toml::Value::Table(profiles)).unwrap();
    fs::write(config_path, serialized).unwrap();
}
