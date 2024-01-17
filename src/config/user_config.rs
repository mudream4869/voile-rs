use serde::{Deserialize, Serialize};

pub fn default_user_config() -> &'static str {
    include_str!("../../configs/user.default.toml")
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub theme: String,

    #[serde(default)]
    pub username: String,
    
    #[serde(default)]
    pub password_salt: String,

    #[serde(default)]
    pub password_sha512: String,
}

impl UserConfig {
    pub fn from_toml<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<UserConfig> {
        let config_str = std::fs::read_to_string(filename)?;
        Ok(toml::from_str(&config_str)?)
    }
}
