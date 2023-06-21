use serde::{Deserialize, Serialize};

pub fn default_system_config(voile_config_dir: std::path::PathBuf) -> std::io::Result<String> {
    let default_toml = include_str!("../../configs/system.default.toml");
    let mut doc: toml_edit::Document = default_toml.parse().unwrap();

    let default_server_data_dir = voile_config_dir.join("server_data");
    let default_data_dir = voile_config_dir.join("books");

    doc["data_dir"] = toml_edit::value(default_data_dir.to_str().unwrap());
    doc["server_data_dir"] = toml_edit::value(default_server_data_dir.to_str().unwrap());

    Ok(doc.to_string())
}

fn default_port() -> u16 {
    8080
}

fn default_ip() -> String {
    "127.0.0.1".to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    #[serde(default = "default_ip")]
    pub ip: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub data_dir: String,

    #[serde(default)]
    pub frontend_dir: String,

    #[serde(default)]
    pub server_data_dir: String,
}

impl SystemConfig {
    pub fn from_dir(voile_config_dir: std::path::PathBuf) -> std::io::Result<SystemConfig> {
        let default_server_data_dir = voile_config_dir.join("server_data");
        let default_data_dir = voile_config_dir.join("books");

        let config_str = std::fs::read_to_string(voile_config_dir.join("system.toml"))?;
        let mut config: SystemConfig = toml::from_str(config_str.as_str())?;

        if config.server_data_dir.is_empty() {
            config.server_data_dir = default_server_data_dir.to_str().unwrap().to_string();
        }

        if config.data_dir.is_empty() {
            config.data_dir = default_data_dir.to_str().unwrap().to_string();
        }

        Ok(config)
    }
}
