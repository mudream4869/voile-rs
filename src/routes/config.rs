use futures_util::TryStreamExt;

use actix_web::{get, post, web, Responder};
use std::sync::{Arc, Mutex};

type ConfigHandler = Arc<Mutex<crate::voile::config::ConfigHandler>>;

#[get("/api/config/system")]
async fn get_system_config(data: web::Data<ConfigHandler>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(data.lock().unwrap().get_system_config()))
}

#[get("/api/config/user")]
async fn get_user_config(data: web::Data<ConfigHandler>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(data.lock().unwrap().get_user_config()?))
}

#[post("/api/config/user")]
async fn set_user_config(
    data: web::Data<ConfigHandler>,
    user_config: web::Json<crate::config::user_config::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if !user_config.name.is_empty() {
        data.lock()
            .unwrap()
            .set_user_name(user_config.name.clone())?;
    }

    if !user_config.theme.is_empty() {
        data.lock()
            .unwrap()
            .set_user_theme(user_config.theme.clone())?;
    }

    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/config/user/avatar")]
async fn get_user_avatar(data: web::Data<ConfigHandler>) -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open(
        data.lock().unwrap().get_user_avatar_path(),
    )?)
}

#[post("/api/config/user/avatar")]
async fn set_user_avatar(
    mut payload: actix_multipart::Multipart,
    data: web::Data<ConfigHandler>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if let Some(field) = payload.try_next().await? {
        data.lock().unwrap().set_user_avatar(field).await?;
    }

    Ok(actix_web::HttpResponse::Ok().into())
}

pub fn configure(cfg: &mut web::ServiceConfig, voile_config_dir: std::path::PathBuf) {
    let user_config = Arc::new(Mutex::new(
        crate::voile::config::ConfigHandler::new(voile_config_dir).unwrap(),
    ));

    cfg.app_data(web::Data::new(user_config))
        .service(get_system_config)
        .service(get_user_avatar)
        .service(set_user_avatar)
        .service(get_user_config)
        .service(set_user_config);
}
