use crate::appstate::appstate::SharedAppState;
use futures_util::TryStreamExt;

use serde::Deserialize;

use actix_web::{get, post, web, Responder};

#[get("/config/system")]
async fn get_system_config(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        app_state.lock().unwrap().config_handler.get_system_config(),
    ))
}

#[get("/config/user")]
async fn get_user_config(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        app_state.lock().unwrap().config_handler.get_user_config()?,
    ))
}

#[post("/config/user/name")]
async fn set_user_name(
    app_state: web::Data<SharedAppState>,
    user_config: web::Json<crate::config::user_config::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    app_state
        .lock()
        .unwrap()
        .config_handler
        .set_user_name(&user_config.name)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[post("/config/user/theme")]
async fn set_user_theme(
    app_state: web::Data<SharedAppState>,
    user_config: web::Json<crate::config::user_config::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    app_state
        .lock()
        .unwrap()
        .config_handler
        .set_user_theme(&user_config.theme)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
struct PasswordBody {
    #[serde(default)]
    pub old_password: String,

    #[serde(default)]
    pub new_password: String,
}

#[post("/config/user/password")]
async fn set_user_password(
    app_state: web::Data<SharedAppState>,
    password_body: web::Json<PasswordBody>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if !app_state
        .lock()
        .unwrap()
        .config_handler
        .auth(&password_body.old_password)?
    {
        return Ok(actix_web::HttpResponse::Unauthorized().finish());
    }

    app_state
        .lock()
        .unwrap()
        .config_handler
        .set_user_password(&password_body.new_password)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[get("/config/user/avatar")]
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

#[post("/config/user/avatar")]
async fn set_user_avatar(
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if let Some(field) = payload.try_next().await? {
        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join("tmp");
        super::util::download_file_from_multipart(field, &tmp_filename).await?;

        app_state
            .lock()
            .unwrap()
            .config_handler
            .set_user_avatar(tmp_filename)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_system_config)
        .service(get_user_avatar)
        .service(set_user_avatar)
        .service(get_user_config)
        .service(set_user_name)
        .service(set_user_theme)
        .service(set_user_password);
}
