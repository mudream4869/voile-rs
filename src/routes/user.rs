use futures_util::TryStreamExt;

use actix_web::{get, post, web, Responder};
use std::sync::{Arc, Mutex};

struct UserConfig {
    user_config: Arc<Mutex<crate::voile::user::User>>,
}

#[get("/api/user/config")]
async fn get_user_config(data: web::Data<UserConfig>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(
        data.user_config.lock().unwrap().get_user_config(),
    ))
}

#[post("/api/user/config")]
async fn set_user_config(
    data: web::Data<UserConfig>,
    user_config: web::Json<crate::voile::user::UserConfig>,
) -> actix_web::Result<actix_web::HttpResponse> {
    data.user_config
        .lock()
        .unwrap()
        .set_user_config(user_config.0)?;
    Ok(actix_web::HttpResponse::Ok().into())
}

#[get("/api/user/avatar")]
async fn get_user_avatar(data: web::Data<UserConfig>) -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open(
        data.user_config.lock().unwrap().get_user_avatar_path(),
    )?)
}

#[post("/api/user/avatar")]
async fn set_user_avatar(
    mut payload: actix_multipart::Multipart,
    data: web::Data<UserConfig>,
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

pub fn configure(cfg: &mut web::ServiceConfig, server_data_dir: String) {
    let user_config = UserConfig {
        user_config: Arc::new(Mutex::new(crate::voile::user::User::new(
            server_data_dir.clone(),
        ))),
    };

    cfg.app_data(web::Data::new(user_config))
        .service(get_user_avatar)
        .service(set_user_avatar)
        .service(get_user_config)
        .service(set_user_config);
}
