pub mod user;
pub mod voile;

use actix_web::{get, post, web, Responder};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct DefaultServerAsset;

fn handle_default_embedded_file(path: &str) -> actix_web::HttpResponse {
    println!("{}", path);
    match DefaultServerAsset::get(path) {
        Some(content) => actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => actix_web::HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[derive(Clone)]
struct AppState {
    voile: Arc<Mutex<voile::Voile>>,
    user_config: Arc<Mutex<user::User>>,
    frontend_dir: Option<String>,
}

#[actix_web::get("/")]
async fn default_index() -> impl Responder {
    handle_default_embedded_file("index.html")
}

#[actix_web::get("/favicon.ico")]
async fn default_favicon() -> impl Responder {
    handle_default_embedded_file("favicon.ico")
}

#[actix_web::get("/assets/{_:.*}")]
async fn default_assets(path: web::Path<String>) -> impl Responder {
    handle_default_embedded_file(format!("assets/{}", path).as_str())
}

#[actix_web::get("/")]
async fn index(data: web::Data<AppState>) -> std::io::Result<actix_files::NamedFile> {
    let index_path: std::path::PathBuf =
        [data.frontend_dir.as_ref().unwrap().as_str(), "index.html"]
            .iter()
            .collect();
    Ok(actix_files::NamedFile::open(index_path)?)
}

#[actix_web::get("/favicon.ico")]
async fn favicon(data: web::Data<AppState>) -> std::io::Result<actix_files::NamedFile> {
    let favicon_path: std::path::PathBuf =
        [data.frontend_dir.as_ref().unwrap().as_str(), "favicon.ico"]
            .iter()
            .collect();
    Ok(actix_files::NamedFile::open(favicon_path)?)
}

#[derive(Serialize)]
struct RespBooks {
    books: Vec<voile::Book>,
}

#[get("/api/books")]
async fn get_books(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let books = data.voile.lock().unwrap().get_books()?;
    Ok(web::Json(RespBooks { books: books }))
}

#[get("/api/books_tags")]
async fn get_books_tags(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let tags = data.voile.lock().unwrap().get_all_book_tags()?;
    Ok(web::Json(tags))
}

#[get("/api/books_types")]
async fn get_books_types(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let types = data.voile.lock().unwrap().get_all_book_types()?;
    Ok(web::Json(types))
}

#[get("/api/books/{book_id}")]
async fn get_book(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book = data.voile.lock().unwrap().get_book(book_id)?;

    Ok(web::Json(book))
}

#[post("/api/books")]
async fn add_book(
    mut payload: actix_multipart::Multipart,
    data: web::Data<AppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    while let Some(item) = payload.next().await {
        let field = item?;
        let res = data.voile.lock().unwrap().add_book(field).await;
        if let Some(err) = res.err() {
            log::warn!("Skip book due to {}", err);
        }
    }

    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/books/{book_id}/book_cover")]
async fn get_book_cover(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_cover_path = data.voile.lock().unwrap().get_book_cover_path(book_id)?;

    Ok(actix_files::NamedFile::open(book_cover_path)?)
}

#[post("/api/books/{book_id}")]
async fn set_book_detail(
    path: web::Path<String>,
    data: web::Data<AppState>,
    book_detail: web::Json<voile::BookDetails>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();

    data.voile
        .lock()
        .unwrap()
        .set_book_detail(book_id, book_detail.0)?;

    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/books/{book_id}/contents/{content_id}")]
async fn get_book_content(
    path: web::Path<(String, usize)>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let (book_id, content_idx) = path.into_inner();

    let content_path = data
        .voile
        .lock()
        .unwrap()
        .get_book_content_path(book_id, content_idx)?;

    Ok(actix_files::NamedFile::open(content_path)?)
}

#[get("/api/user/config")]
async fn get_user_config(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        data.user_config.lock().unwrap().get_user_config(),
    ))
}

#[post("/api/user/config")]
async fn set_user_config(
    data: web::Data<AppState>,
    user_config: web::Json<user::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    data.user_config
        .lock()
        .unwrap()
        .set_user_config(user_config.0)?;
    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/user/avatar")]
async fn get_user_avatar(data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open(
        data.user_config.lock().unwrap().get_user_avatar_path(),
    )?)
}

#[post("/api/user/avatar")]
async fn set_user_avatar(
    mut payload: actix_multipart::Multipart,
    data: web::Data<AppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if let Some(field) = payload.try_next().await? {
        data.user_config
            .lock()
            .unwrap()
            .set_user_avatar(field)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/user/book_proc/{book_id}")]
async fn get_book_proc(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_proc = data.voile.lock().unwrap().get_book_proc(book_id)?;

    Ok(web::Json(book_proc))
}

#[post("/api/user/book_proc/{book_id}")]
async fn set_book_proc(
    path: web::Path<String>,
    data: web::Data<AppState>,
    book_proc: web::Json<voile::BookProc>,
) -> actix_web::Result<impl Responder> {
    // TODO: error handling
    let book_id = path.into_inner();

    data.voile
        .lock()
        .unwrap()
        .set_book_proc(book_id, &book_proc.0)?;

    Ok(book_proc)
}

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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

    let data = AppState {
        voile: Arc::new(Mutex::new(
            voile::Voile::new(conf.data_dir.clone()).unwrap(),
        )),
        user_config: Arc::new(Mutex::new(user::User::new(server_data_dir.clone()))),
        frontend_dir: conf.frontend_dir.clone(),
    };

    log::info!("Listen on: http://{}:{}", conf.ip.clone(), conf.port);

    actix_web::HttpServer::new(move || {
        let app = actix_web::App::new()
            .app_data(web::Data::new(data.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(get_books)
            .service(get_books_tags)
            .service(get_books_types)
            .service(get_book)
            .service(add_book)
            .service(get_book_cover)
            .service(set_book_detail)
            .service(get_book_content)
            .service(get_user_avatar)
            .service(set_user_avatar)
            .service(get_user_config)
            .service(set_user_config)
            .service(get_book_proc)
            .service(set_book_proc);

        match conf.frontend_dir.as_ref() {
            Some(frontend_dir) => {
                let assets_path: std::path::PathBuf =
                    [frontend_dir.as_str(), "assets"].iter().collect();
                app.service(index)
                    .service(favicon)
                    .service(actix_files::Files::new("/assets", assets_path).show_files_listing())
            }
            None => app
                .service(default_index)
                .service(default_favicon)
                .service(default_assets),
        }
    })
    .bind((conf.ip, conf.port))?
    .run()
    .await
}
