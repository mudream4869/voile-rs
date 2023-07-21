use actix_web::web;

struct UserFrontend {
    frontend_dir: std::path::PathBuf,
}

#[actix_web::get("/")]
async fn index(data: web::Data<UserFrontend>) -> std::io::Result<actix_files::NamedFile> {
    let index_path = data.frontend_dir.join("index.html");
    Ok(actix_files::NamedFile::open(index_path)?)
}

#[actix_web::get("/favicon.ico")]
async fn favicon(data: web::Data<UserFrontend>) -> std::io::Result<actix_files::NamedFile> {
    let favicon_path = data.frontend_dir.join("favicon.ico");
    Ok(actix_files::NamedFile::open(favicon_path)?)
}

pub fn configure(cfg: &mut web::ServiceConfig, frontend_dir: &str) {
    let user_frontend = UserFrontend {
        frontend_dir: frontend_dir.into(),
    };

    let assets_path = user_frontend.frontend_dir.join("assets");

    cfg.app_data(web::Data::new(user_frontend))
        .service(index)
        .service(favicon)
        .service(actix_files::Files::new("/assets", assets_path).show_files_listing());
}
