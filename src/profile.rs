use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub email: String,
    pub ssh_key_path: String,
    pub host_alias: String,
}

impl TryFrom<toml::Value> for Profile {
    type Error = String;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or("Expected a table")?;
        let username = table.get("username")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'username' field")?
            .to_string();
        let name = table.get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'name' field")?
            .to_string();
        let email = table.get("email")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'email' field")?
            .to_string();
        let ssh_key_path = table.get("ssh_key_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'ssh_key_path' field")?
            .to_string();
        let host_alias = table.get("host_alias")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'host_alias' field")?
            .to_string();

        Ok(Profile { name, username, email, ssh_key_path, host_alias })
    }
}
