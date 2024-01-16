use sha2::{digest::Digest, Sha512};
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ConfigHandler {
    user_config_filename: PathBuf,
    system_config: crate::config::system_config::SystemConfig,
}

impl ConfigHandler {
    pub fn new(voile_config_dir: PathBuf) -> std::io::Result<ConfigHandler> {
        let user_config_filename = voile_config_dir.join("user.toml");

        let system_config = crate::config::system_config::SystemConfig::from_dir(voile_config_dir)?;

        let config_handler = ConfigHandler {
            user_config_filename: user_config_filename,
            system_config: system_config,
        };

        Ok(config_handler)
    }

    pub fn get_user_config(&self) -> std::io::Result<crate::config::user_config::UserConfig> {
        crate::config::user_config::UserConfig::from_toml(&self.user_config_filename)
    }

    pub fn get_system_config(&self) -> crate::config::system_config::SystemConfig {
        self.system_config.clone()
    }

    fn edit_user_config(&self, key: &str, value: &str) -> Result<()> {
        let config_str = std::fs::read_to_string(&self.user_config_filename)?;
        let mut doc: toml_edit::Document = config_str.parse()?;
        doc[key] = toml_edit::value(value);
        std::fs::write(&self.user_config_filename, doc.to_string())?;
        Ok(())
    }

    pub fn set_user_name(&self, name: &str) -> Result<()> {
        self.edit_user_config("name", name)
    }

    pub fn set_user_theme(&self, theme: &str) -> Result<()> {
        self.edit_user_config("theme", theme)
    }

    pub fn auth(&self, username: &str, password: &str) -> Result<bool> {
        let user_config = self.get_user_config()?;

        if user_config.username != username {
            return Ok(false);
        }

        let mut hasher = Sha512::new();
        hasher.update(password.as_bytes());
        hasher.update(user_config.password_salt.as_bytes());

        let hex_res = hex::encode(hasher.finalize().as_slice());

        if user_config.password_sha512 != hex_res {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn get_user_avatar_path(&self) -> PathBuf {
        [&self.system_config.server_data_dir, "avatar.png"]
            .iter()
            .collect()
    }

    pub async fn set_user_avatar(&self, filesource: PathBuf) -> Result<()> {
        let filepath = self.get_user_avatar_path();
        crate::voile::util::move_file(filesource, filepath)?;
        Ok(())
    }
}
