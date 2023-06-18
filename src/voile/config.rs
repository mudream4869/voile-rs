use std::io::Write;

use futures_util::TryStreamExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ConfigHandler {
    user_config_filename: std::path::PathBuf,
    system_config: crate::config::system_config::SystemConfig,
}

impl ConfigHandler {
    pub fn new(voile_config_dir: std::path::PathBuf) -> std::io::Result<ConfigHandler> {
        let mut user_config_filename = voile_config_dir.clone();
        user_config_filename.push("user.toml");

        let system_config = crate::config::system_config::SystemConfig::from_dir(voile_config_dir)?;

        let config_handler = ConfigHandler {
            user_config_filename: user_config_filename,
            system_config: system_config,
        };

        Ok(config_handler)
    }

    pub fn get_user_config(&self) -> std::io::Result<crate::config::user_config::UserConfig> {
        crate::config::user_config::UserConfig::from_toml(self.user_config_filename.clone())
    }

    pub fn get_system_config(&self) -> crate::config::system_config::SystemConfig {
        self.system_config.clone()
    }

    fn edit_user_config(&self, key: &str, value: String) -> Result<()> {
        let config_str = std::fs::read_to_string(&self.user_config_filename)?;
        let mut doc: toml_edit::Document = config_str.parse()?;
        doc[key] = toml_edit::value(value);
        std::fs::write(&self.user_config_filename, doc.to_string())?;
        Ok(())
    }

    pub fn set_user_name(&self, name: String) -> Result<()> {
        self.edit_user_config("name", name)
    }

    pub fn set_user_theme(&self, theme: String) -> Result<()> {
        self.edit_user_config("theme", theme)
    }

    pub fn get_user_avatar_path(&self) -> std::path::PathBuf {
        [self.system_config.server_data_dir.as_str(), "avatar.png"]
            .iter()
            .collect()
    }

    pub async fn set_user_avatar(&self, mut field: actix_multipart::Field) -> Result<()> {
        // TODO: refine error
        let filepath = self.get_user_avatar_path();

        // File::create is blocking operation, use threadpool
        let mut f = actix_web::web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = actix_web::web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }

        Ok(())
    }
}
