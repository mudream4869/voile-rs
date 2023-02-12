use std::io::Write;

use futures_util::TryStreamExt;
pub struct User {
    server_data_dir: String,
}

impl User {
    pub fn new(server_data_dir: String) -> User {
        User {
            server_data_dir: server_data_dir,
        }
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
        let filepath: std::path::PathBuf = [self.server_data_dir.as_str(), "avatar.png"]
            .iter()
            .collect();

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
