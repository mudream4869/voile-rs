type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use std::sync::{Arc, Mutex};

pub type SharedAppState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub voile: crate::voile::voile::Voile,
    pub config_handler: crate::voile::config::ConfigHandler,
}

impl AppState {
    pub fn new(voile_config_dir: std::path::PathBuf) -> Result<AppState> {
        Ok(AppState {
            voile: crate::voile::voile::Voile::new(voile_config_dir.clone())?,
            config_handler: crate::voile::config::ConfigHandler::new(voile_config_dir.clone())?,
        })
    }
}
