extern crate voile;

use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    voile: Arc<Mutex<voile::Voile>>,
    frontend_dir: String,
}

async fn index(data: web::Data<AppState>) -> std::io::Result<actix_files::NamedFile> {
    let index_path: std::path::PathBuf =
        [data.frontend_dir.as_str(), "index.html"].iter().collect();
    Ok(actix_files::NamedFile::open(index_path)?)
}

async fn favicon(data: web::Data<AppState>) -> std::io::Result<actix_files::NamedFile> {
    let favicon_path: std::path::PathBuf =
        [data.frontend_dir.as_str(), "favicon.ico"].iter().collect();
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

#[get("/api/books/{book_id}")]
async fn get_book(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book = data.voile.lock().unwrap().get_book(book_id)?;

    Ok(web::Json(book))
}

#[post("/api/books/{book_id}")]
async fn set_book_detail(
    path: web::Path<String>,
    data: web::Data<AppState>,
    book_detail: web::Json<voile::BookDetails>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    data.voile
        .lock()
        .unwrap()
        .set_book_detail(book_id, book_detail.0)?;

    Ok("")
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
        .get_book_content(book_id, content_idx)?;
    Ok(actix_files::NamedFile::open(content_path)?)
}

#[get("/api/user/book_proc/{book_id}")]
async fn get_book_proc(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let content_idx = data.voile.lock().unwrap().get_book_proc(book_id)?;

    Ok(content_idx.to_string())
}

#[post("/api/user/book_proc/{book_id}")]
async fn set_book_proc(
    path: web::Path<String>,
    data: web::Data<AppState>,
    body: web::Bytes,
) -> actix_web::Result<impl Responder> {
    // TODO: error handling
    let book_id = path.into_inner();

    let content_idx = std::str::from_utf8(&body)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    data.voile
        .lock()
        .unwrap()
        .set_book_proc(book_id, content_idx)?;

    Ok(content_idx.to_string())
}

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
    data_dir: String,
    frontend_dir: String,
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

    let config_filename: std::path::PathBuf;
    if args.len() == 1 {
        match home::home_dir() {
            Some(mut home_dir) => {
                home_dir.push(".voile");
                home_dir.push("config.toml");
                config_filename = home_dir.clone();
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

    let conf = Config::from_filename(&config_filename)?;
    app(conf)
}

#[actix_web::main]
async fn app(conf: Config) -> std::io::Result<()> {
    let data = AppState {
        voile: Arc::new(Mutex::new(
            voile::Voile::new(conf.data_dir.clone()).unwrap(),
        )),
        frontend_dir: conf.frontend_dir.clone(),
    };

    log::info!("Listen on: http://{}:{}", conf.ip.clone(), conf.port);

    actix_web::HttpServer::new(move || {
        let assets_path: std::path::PathBuf =
            [conf.frontend_dir.as_str(), "assets"].iter().collect();

        actix_web::App::new()
            .app_data(web::Data::new(data.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(get_books)
            .service(get_book)
            .service(set_book_detail)
            .service(get_book_content)
            .service(get_book_proc)
            .service(set_book_proc)
            .route("/", web::get().to(index))
            .route("/favicon.ico", web::get().to(favicon))
            .service(actix_files::Files::new("/assets", assets_path).show_files_listing())
    })
    .bind((conf.ip, conf.port))?
    .run()
    .await
}
