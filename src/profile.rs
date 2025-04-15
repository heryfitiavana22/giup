use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub email: String,
    pub ssh_key_path: String,
    pub host_alias: String,
}
