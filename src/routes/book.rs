use actix_web::{delete, get, post, web, Responder};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt;

use serde::Serialize;
use std::sync::{Arc, Mutex};
struct Voile {
    voile: Arc<Mutex<crate::voile::voile::Voile>>,
}

#[derive(Serialize)]
struct RespBooks {
    books: Vec<crate::voile::voile::Book>,
}

#[get("/api/books")]
async fn get_books(data: web::Data<Voile>) -> actix_web::Result<impl Responder> {
    let books = data.voile.lock().unwrap().get_books()?;
    Ok(web::Json(RespBooks { books: books }))
}

#[get("/api/books_tags")]
async fn get_books_tags(data: web::Data<Voile>) -> actix_web::Result<impl Responder> {
    let tags = data.voile.lock().unwrap().get_all_book_tags()?;
    Ok(web::Json(tags))
}

#[get("/api/books_types")]
async fn get_books_types(data: web::Data<Voile>) -> actix_web::Result<impl Responder> {
    let types = data.voile.lock().unwrap().get_all_book_types()?;
    Ok(web::Json(types))
}

#[get("/api/books/{book_id}")]
async fn get_book(
    path: web::Path<String>,
    data: web::Data<Voile>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book = data.voile.lock().unwrap().get_book(book_id)?;

    Ok(web::Json(book))
}

#[delete("/api/books/{book_id}")]
async fn delete_book(
    path: web::Path<String>,
    data: web::Data<Voile>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();

    data.voile.lock().unwrap().delete_book(book_id)?;

    Ok(actix_web::HttpResponse::Ok().into())
}

#[post("/api/books")]
async fn add_book(
    mut payload: actix_multipart::Multipart,
    data: web::Data<Voile>,
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
    data: web::Data<Voile>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_cover_path = data.voile.lock().unwrap().get_book_cover_path(book_id)?;

    Ok(actix_files::NamedFile::open(book_cover_path)?)
}

#[post("/api/books/{book_id}/book_cover")]
async fn set_book_cover(
    path: web::Path<String>,
    mut payload: actix_multipart::Multipart,
    data: web::Data<Voile>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();
    if let Some(field) = payload.try_next().await? {
        data.voile
            .lock()
            .unwrap()
            .set_book_cover(book_id, field)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().into())
}

#[post("/api/books/{book_id}")]
async fn set_book_detail(
    path: web::Path<String>,
    data: web::Data<Voile>,
    book_detail: web::Json<crate::voile::voile::BookDetails>,
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
    data: web::Data<Voile>,
) -> actix_web::Result<impl Responder> {
    let (book_id, content_idx) = path.into_inner();

    let content_path = data
        .voile
        .lock()
        .unwrap()
        .get_book_content_path(book_id, content_idx)?;

    Ok(actix_files::NamedFile::open(content_path)?)
}

#[get("/api/user/book_proc/{book_id}")]
async fn get_book_proc(
    path: web::Path<String>,
    data: web::Data<Voile>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_proc = data.voile.lock().unwrap().get_book_proc(book_id)?;

    Ok(web::Json(book_proc))
}

#[post("/api/user/book_proc/{book_id}")]
async fn set_book_proc(
    path: web::Path<String>,
    data: web::Data<Voile>,
    book_proc: web::Json<crate::voile::voile::BookProc>,
) -> actix_web::Result<impl Responder> {
    // TODO: error handling
    let book_id = path.into_inner();

    data.voile
        .lock()
        .unwrap()
        .set_book_proc(book_id, &book_proc.0)?;

    Ok(book_proc)
}

pub fn configure(cfg: &mut web::ServiceConfig, data_dir: String) {
    let voile = Voile {
        voile: Arc::new(Mutex::new(
            crate::voile::voile::Voile::new(data_dir.clone()).unwrap(),
        )),
    };

    cfg.app_data(web::Data::new(voile))
        .service(get_books)
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
