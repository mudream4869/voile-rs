use path_absolutize::Absolutize;

pub mod config;
pub mod routes;
pub mod voile;

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let args: Vec<String> = std::env::args().collect();

    let voile_config_dir: std::path::PathBuf;

    if args.len() == 1 {
        match dirs::config_dir() {
            Some(config_dir) => {
                voile_config_dir = config_dir.join("Voile");
            }
            None => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
            }
        }
    } else if args.len() == 2 {
        voile_config_dir = std::path::PathBuf::from(args[1].as_str());
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
    }

    let config_dir = std::path::Path::new(&voile_config_dir)
        .absolutize()
        .unwrap();

    config::config::prepare_config_dir(std::path::PathBuf::from(config_dir.clone()))?;

    log::info!("Config dir:{:?}", config_dir);

    app(voile_config_dir)
}

#[actix_web::main]
async fn app(voile_config_dir: std::path::PathBuf) -> std::io::Result<()> {
    let conf = config::system_config::SystemConfig::from_dir(voile_config_dir.clone())?;

    log::info!("Listen on: http://{}:{}", conf.ip.clone(), conf.port);

    actix_web::HttpServer::new(move || {
        let app = actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|s| routes::book::configure(s, conf.data_dir.clone()))
            .configure(|s| routes::config::configure(s, voile_config_dir.clone()));

        if conf.frontend_dir.is_empty() {
            app.configure(routes::default_frontend::configure)
        } else {
            app.configure(|s| routes::userdefine_frontend::configure(s, conf.frontend_dir.clone()))
        }
    })
    .bind((conf.ip, conf.port))?
    .run()
    .await
}
