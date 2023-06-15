pub mod routes;
pub mod voile;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
    data_dir: String,
    frontend_dir: Option<String>,
    server_data_dir: Option<String>,
}

impl Config {
    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<Config> {
        log::info!("Config file: {}", filename.as_ref().display());

        let detail_str = std::fs::read_to_string(filename)?;
        let detail: Config = toml::from_str(detail_str.as_str())?;
        Ok(detail)
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let args: Vec<String> = std::env::args().collect();

    let mut config_filename: std::path::PathBuf;
    let mut default_server_data_dir: std::path::PathBuf = std::env::current_dir()?;
    default_server_data_dir.push("data");

    if args.len() == 1 {
        match home::home_dir() {
            Some(home_dir) => {
                config_filename = home_dir.clone();
                config_filename.push(".voile");
                config_filename.push("config.toml");

                default_server_data_dir = home_dir.clone();
                default_server_data_dir.push(".voile");
                default_server_data_dir.push("data");
            }
            None => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
            }
        }
    } else if args.len() == 2 {
        config_filename = [args[1].as_str()].iter().collect();
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
    }

    let mut conf = Config::from_filename(&config_filename)?;
    if conf.server_data_dir.is_none() {
        conf.server_data_dir = Some(default_server_data_dir.to_str().unwrap().to_string());
    }
    app(conf)
}

#[actix_web::main]
async fn app(conf: Config) -> std::io::Result<()> {
    let server_data_dir = conf.server_data_dir.unwrap();

    log::info!("Listen on: http://{}:{}", conf.ip.clone(), conf.port);

    actix_web::HttpServer::new(move || {
        let app = actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|s| routes::book::configure(s, conf.data_dir.clone()))
            .configure(|s| routes::user::configure(s, server_data_dir.clone()));

        match conf.frontend_dir.as_ref() {
            Some(frontend_dir) => app
                .configure(|s| routes::userdefine_frontend::configure(s, frontend_dir.to_string())),
            None => app.configure(routes::default_frontend::configure),
        }
    })
    .bind((conf.ip, conf.port))?
    .run()
    .await
}
