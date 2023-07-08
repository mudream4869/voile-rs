use actix_web::{delete, get, post, web, Responder};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt;

use crate::appstate::appstate::SharedAppState;
use serde::Serialize;

#[derive(Serialize)]
struct RespBooks {
    books: Vec<crate::voile::voile::Book>,
}

#[get("/api/books")]
async fn get_books(app_state: web::Data<SharedAppState>) -> actix_web::Result<impl Responder> {
    let books = app_state.lock().unwrap().voile.get_books()?;
    Ok(web::Json(RespBooks { books }))
}

#[get("/api/books_tags")]
async fn get_books_tags(app_state: web::Data<SharedAppState>) -> actix_web::Result<impl Responder> {
    let tags = app_state.lock().unwrap().voile.get_all_book_tags()?;
    Ok(web::Json(tags))
}

#[get("/api/books_types")]
async fn get_books_types(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let types = app_state.lock().unwrap().voile.get_all_book_types()?;
    Ok(web::Json(types))
}

#[get("/api/books/{book_id}")]
async fn get_book(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book = app_state.lock().unwrap().voile.get_book(book_id)?;

    Ok(web::Json(book))
}

#[delete("/api/books/{book_id}")]
async fn delete_book(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();

    app_state.lock().unwrap().voile.delete_book(book_id)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[post("/api/books")]
async fn add_book(
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    while let Some(item) = payload.next().await {
        let field = item?;

        let filename = if let Some(filename) = field.content_disposition().get_filename() {
            filename.to_string()
        } else {
            log::warn!("Skip book due to no filename");
            continue;
        };

        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join(filename.clone());
        crate::routes::util::download_file_from_multipart(field, &tmp_filename).await?;

        let res = app_state
            .lock()
            .unwrap()
            .voile
            .add_book(filename, tmp_filename)
            .await;

        if let Err(err) = res {
            log::warn!("Skip book due to {}", err);
        }
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[get("/api/books/{book_id}/book_cover")]
async fn get_book_cover(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_cover_path = app_state
        .lock()
        .unwrap()
        .voile
        .get_book_cover_path(book_id)?;

    Ok(actix_files::NamedFile::open(book_cover_path)?)
}

#[post("/api/books/{book_id}/book_cover")]
async fn set_book_cover(
    path: web::Path<String>,
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();
    if let Some(field) = payload.try_next().await? {
        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join("tmp");
        crate::routes::util::download_file_from_multipart(field, &tmp_filename).await?;

        app_state
            .lock()
            .unwrap()
            .voile
            .set_book_cover(book_id, tmp_filename)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[post("/api/books/{book_id}")]
async fn set_book_detail(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
    book_detail: web::Json<crate::voile::voile::BookDetails>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();

    app_state
        .lock()
        .unwrap()
        .voile
        .set_book_detail(book_id, book_detail.0)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[get("/api/books/{book_id}/contents/{content_id}")]
async fn get_book_content(
    path: web::Path<(String, usize)>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let (book_id, content_idx) = path.into_inner();

    let content_path = app_state
        .lock()
        .unwrap()
        .voile
        .get_book_content_path(book_id, content_idx)?;

    Ok(actix_files::NamedFile::open(content_path)?)
}

#[get("/api/user/book_proc/{book_id}")]
async fn get_book_proc(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_proc = app_state.lock().unwrap().voile.get_book_proc(book_id)?;

    Ok(web::Json(book_proc))
}

#[post("/api/user/book_proc/{book_id}")]
async fn set_book_proc(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
    book_proc: web::Json<crate::voile::voile::BookProc>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    app_state
        .lock()
        .unwrap()
        .voile
        .set_book_proc(book_id, &book_proc.0)?;

    Ok(book_proc)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books)
        .service(get_books_tags)
        .service(get_books_types)
        .service(get_book)
        .service(add_book)
        .service(delete_book)
        .service(get_book_cover)
        .service(set_book_cover)
        .service(set_book_detail)
        .service(get_book_content)
        .service(get_book_proc)
        .service(set_book_proc);
}
