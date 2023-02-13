use std::io::Write;

use serde::{Deserialize, Serialize};

use futures_util::TryStreamExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

impl UserConfig {
    pub fn new() -> UserConfig {
        UserConfig {
            name: None,
            theme: None,
        }
    }

    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<UserConfig> {
        let detail_str = std::fs::read_to_string(filename)?;
        let detail: UserConfig = serde_json::from_str(detail_str.as_str())?;
        Ok(detail)
    }

    pub fn write_to_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> std::io::Result<()> {
        let detail_str = serde_json::to_string_pretty(&self)?;
        std::fs::write(filename, detail_str)?;
        Ok(())
    }
}

pub struct User {
    server_data_dir: String,
    user_config: UserConfig,
}

impl User {
    pub fn new(server_data_dir: String) -> User {
        let mut user = User {
            server_data_dir: server_data_dir,
            user_config: UserConfig::new(),
        };

        if let Ok(user_config) = UserConfig::from_filename(user.get_user_config_file_path()) {
            user.user_config = user_config
        }

        user
    }

    fn get_user_config_file_path(&self) -> std::path::PathBuf {
        [self.server_data_dir.as_str(), "user.json"]
            .iter()
            .collect()
    }

    pub fn get_user_config(&self) -> UserConfig {
        self.user_config.clone()
    }

    pub fn set_user_config(&mut self, user_config: UserConfig) -> std::io::Result<()> {
        user_config.write_to_filename(self.get_user_config_file_path())?;
        self.user_config = user_config;

        Ok(())
    }

    pub fn get_user_avatar_path(&self) -> std::path::PathBuf {
        [self.server_data_dir.as_str(), "avatar.png"]
            .iter()
            .collect()
    }

    pub async fn set_user_avatar(
        &self,
        mut field: actix_multipart::Field,
    ) -> actix_web::Result<()> {
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
