use super::system_config::default_system_config;
use super::user_config::default_user_config;

/// Prepare full config directory hierarchy.
///
/// The `voile_config_dir` parameter represents the path to the directory where the
/// configuration files and folders will be created.
///
/// The following hierarchy will be created within the `voile_config_dir`:
/// - `system.toml`: The system configuration file.
/// - `user.toml`: The user configuration file.
/// - `server_data`: The default folder for storing server data.
/// - `books`: The default folder for storing books.
///
/// This function creates the necessary directories and writes the default
/// configuration files if they don't already exist.
///
/// # Errors
///
/// This function may return an `std::io::Error` if there is a problem creating
/// the directories or writing the files.
pub fn prepare_config_dir(voile_config_dir: std::path::PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(&voile_config_dir)?;

    let system_config_filename = voile_config_dir.join("system.toml");

    if !std::path::Path::new(&system_config_filename).exists() {
        std::fs::write(
            &system_config_filename,
            default_system_config(voile_config_dir.clone())?,
        )?;

        std::fs::create_dir_all(voile_config_dir.join("server_data"))?;
        std::fs::create_dir_all(voile_config_dir.join("books"))?;
    }

    let user_config_filename = voile_config_dir.join("user.toml");

    if !std::path::Path::new(&user_config_filename).exists() {
        std::fs::write(&user_config_filename, default_user_config())?;
    }

    Ok(())
}
