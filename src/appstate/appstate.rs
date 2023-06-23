type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use std::sync::{Arc, Mutex};

pub type SharedAppState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub voile: crate::voile::voile::Voile,
    pub config_handler: crate::voile::config::ConfigHandler,
}

impl AppState {
    pub fn new(voile_config_dir: std::path::PathBuf, data_dir: String) -> Result<AppState> {
        Ok(AppState {
            voile: crate::voile::voile::Voile::new(data_dir)?,
            config_handler: crate::voile::config::ConfigHandler::new(voile_config_dir)?,
        })
    }
}
