use rust_embed::RustEmbed;

use actix_web::{web, Responder};

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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(default_index)
        .service(default_favicon)
        .service(default_assets);
}
