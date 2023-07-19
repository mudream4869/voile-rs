use actix_web::web;

struct UserFrontend {
    frontend_dir: String,
}

#[actix_web::get("/")]
async fn index(data: web::Data<UserFrontend>) -> std::io::Result<actix_files::NamedFile> {
    let index_path: std::path::PathBuf = [&data.frontend_dir, "index.html"].iter().collect();
    Ok(actix_files::NamedFile::open(index_path)?)
}

#[actix_web::get("/favicon.ico")]
async fn favicon(data: web::Data<UserFrontend>) -> std::io::Result<actix_files::NamedFile> {
    let favicon_path: std::path::PathBuf = [&data.frontend_dir, "favicon.ico"].iter().collect();
    Ok(actix_files::NamedFile::open(favicon_path)?)
}

pub fn configure(cfg: &mut web::ServiceConfig, frontend_dir: &str) {
    let assets_path: std::path::PathBuf = [frontend_dir, "assets"].iter().collect();

    let user_frontend = UserFrontend {
        frontend_dir: frontend_dir.to_string(),
    };

    cfg.app_data(web::Data::new(user_frontend))
        .service(index)
        .service(favicon)
        .service(actix_files::Files::new("/assets", assets_path).show_files_listing());
}
