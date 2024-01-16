use path_absolutize::Absolutize;

use std::sync::{Arc, Mutex};

pub mod appstate;
pub mod config;
pub mod routes;
pub mod user;
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
        voile_config_dir = std::path::PathBuf::from(&args[1]);
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
    }

    let config_dir = voile_config_dir.as_path().absolutize().unwrap();

    config::config::prepare_config_dir(std::path::PathBuf::from(config_dir.clone()))?;

    log::info!("Config dir:{:?}", config_dir);

    app(voile_config_dir)
}

fn configure_frontend(cfg: &mut actix_web::web::ServiceConfig, frontend_dir: &String) {
    if frontend_dir.is_empty() {
        routes::default_frontend::configure(cfg);
    } else {
        routes::userdefine_frontend::configure(cfg, frontend_dir);
    }
}

#[actix_web::main]
async fn app(voile_config_dir: std::path::PathBuf) -> std::io::Result<()> {
    let sys_conf = config::system_config::SystemConfig::from_dir(voile_config_dir.clone())?;

    // TODO: remove unwrap
    let app_state: crate::appstate::appstate::SharedAppState = Arc::new(Mutex::new(
        crate::appstate::appstate::AppState::new(voile_config_dir.clone()).unwrap(),
    ));

    let serve_url = format!("http://{}:{}", &sys_conf.ip, sys_conf.port);

    log::info!("Listen on: {}", serve_url);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .configure(routes::user::configure)
            .configure(|s| configure_frontend(s, &sys_conf.frontend_dir))
            .wrap(
                // TODO: setting key from config
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    actix_web::cookie::Key::from(&[0; 64]),
                )
                .cookie_secure(false)
                .build(),
            )
            .service(
                actix_web::web::scope("/api")
                    .wrap(routes::user::Authentication)
                    .configure(routes::book::configure)
                    .configure(routes::config::configure),
            )
    })
    .bind((sys_conf.ip, sys_conf.port))?
    .run();

    if sys_conf.open_browser {
        if let Err(err) = open::that(&serve_url) {
            eprintln!("An error occurred when opening '{}': {}", serve_url, err);
        }
    }

    server.await
}
