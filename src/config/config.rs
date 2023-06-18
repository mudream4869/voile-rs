use super::system_config::default_system_config;
use super::user_config::default_user_config;

pub fn prepare_config_dir(voile_config_dir: std::path::PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(&voile_config_dir)?;

    let mut system_config_filename = voile_config_dir.clone();
    system_config_filename.push("system.toml");

    if !std::path::Path::new(&system_config_filename).exists() {
        std::fs::write(
            &system_config_filename,
            default_system_config(voile_config_dir.clone())?,
        )?;

        let mut default_server_data_dir = voile_config_dir.clone();
        default_server_data_dir.push("server_data");

        std::fs::create_dir_all(default_server_data_dir)?;

        let mut default_data_dir = voile_config_dir.clone();
        default_data_dir.push("books");

        std::fs::create_dir_all(default_data_dir)?;
    }

    let mut user_config_filename = voile_config_dir.clone();
    user_config_filename.push("user.toml");

    if !std::path::Path::new(&user_config_filename).exists() {
        std::fs::write(&user_config_filename, default_user_config())?;
    }

    Ok(())
}
