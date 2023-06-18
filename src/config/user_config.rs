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
}

impl UserConfig {
    pub fn from_toml<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<UserConfig> {
        let detail_str = std::fs::read_to_string(filename)?;
        let detail: UserConfig = toml::from_str(&detail_str)?;
        Ok(detail)
    }
}
