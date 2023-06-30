use crate::appstate::appstate::SharedAppState;
use futures_util::TryStreamExt;

use actix_web::{get, post, web, Responder};

#[get("/api/config/system")]
async fn get_system_config(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        app_state.lock().unwrap().config_handler.get_system_config(),
    ))
}

#[get("/api/config/user")]
async fn get_user_config(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        app_state.lock().unwrap().config_handler.get_user_config()?,
    ))
}

#[post("/api/config/user")]
async fn set_user_config(
    app_state: web::Data<SharedAppState>,
    user_config: web::Json<crate::config::user_config::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if !user_config.name.is_empty() {
        app_state
            .lock()
            .unwrap()
            .config_handler
            .set_user_name(user_config.name.clone())?;
    }

    if !user_config.theme.is_empty() {
        app_state
            .lock()
            .unwrap()
            .config_handler
            .set_user_theme(user_config.theme.clone())?;
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[get("/api/config/user/avatar")]
async fn get_user_avatar(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open(
        app_state
            .lock()
            .unwrap()
            .config_handler
            .get_user_avatar_path(),
    )?)
}

#[post("/api/config/user/avatar")]
async fn set_user_avatar(
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if let Some(field) = payload.try_next().await? {
        app_state
            .lock()
            .unwrap()
            .config_handler
            .set_user_avatar(field)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_system_config)
        .service(get_user_avatar)
        .service(set_user_avatar)
        .service(get_user_config)
        .service(set_user_config);
}