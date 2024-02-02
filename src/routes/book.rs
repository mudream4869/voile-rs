use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{delete, get, post, web, Responder};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt;

use crate::appstate::appstate::SharedAppState;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GetBooksParams {
    query: Option<String>,
}

#[derive(Serialize)]
struct RespBooks {
    books: Vec<crate::voile::voile::Book>,
}

#[get("/books")]
async fn get_books(
    app_state: web::Data<SharedAppState>,
    params: web::Query<GetBooksParams>,
) -> actix_web::Result<impl Responder> {
    let mut books = app_state.lock().unwrap().voile.get_books()?;

    // TODO: design a search engine.
    if let Some(query) = &params.query {
        let mut res_books = vec![];
        for book in books {
            if book.title.contains(query) {
                res_books.push(book)
            }
        }

        books = res_books;
    }

    Ok(web::Json(RespBooks { books }))
}

#[get("/books_tags")]
async fn get_books_tags(app_state: web::Data<SharedAppState>) -> actix_web::Result<impl Responder> {
    let tags = app_state.lock().unwrap().voile.get_all_book_tags()?;
    Ok(web::Json(tags))
}

#[get("/books_types")]
async fn get_books_types(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let types = app_state.lock().unwrap().voile.get_all_book_types()?;
    Ok(web::Json(types))
}

#[get("/books_langs")]
async fn get_books_langs(
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let langs = app_state.lock().unwrap().voile.get_all_book_langs()?;
    Ok(web::Json(langs))
}

#[get("/books/{book_id}")]
async fn get_book(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book = app_state.lock().unwrap().voile.get_book(&book_id)?;

    Ok(web::Json(book))
}

#[delete("/books/{book_id}")]
async fn delete_book(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();

    app_state.lock().unwrap().voile.delete_book(&book_id)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[post("/books")]
async fn add_book(
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    if let Some(item) = payload.next().await {
        let field = item?;

        let filename = if let Some(filename) = field.content_disposition().get_filename() {
            filename.to_string()
        } else {
            return Err(ErrorBadRequest(crate::voile::errors::NotExist(
                "filename".to_string(),
            )));
        };

        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join(&filename);
        super::util::download_file_from_multipart(field, &tmp_filename).await?;

        let res = app_state
            .lock()
            .unwrap()
            .voile
            .add_book(&filename, tmp_filename)
            .await;

        if let Err(err) = res {
            return Err(ErrorInternalServerError(err));
        }

        return Ok(actix_web::HttpResponse::Ok().finish());
    }

    return Err(ErrorBadRequest(crate::voile::errors::NotExist(
        "file".to_string(),
    )));
}

#[get("/books/{book_id}/book_cover")]
async fn get_book_cover(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_cover_path = app_state
        .lock()
        .unwrap()
        .voile
        .get_book_cover_path(&book_id)?;

    Ok(actix_files::NamedFile::open(book_cover_path)?)
}

#[post("/books/{book_id}/book_cover")]
async fn set_book_cover(
    path: web::Path<String>,
    mut payload: actix_multipart::Multipart,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let book_id = path.into_inner();
    if let Some(field) = payload.try_next().await? {
        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join("tmp");
        super::util::download_file_from_multipart(field, &tmp_filename).await?;

        app_state
            .lock()
            .unwrap()
            .voile
            .set_book_cover(&book_id, tmp_filename)
            .await?;
    }

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[post("/books/{book_id}")]
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
        .set_book_detail(&book_id, book_detail.0)?;

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[get("/books/{book_id}/contents/{content_id}")]
async fn get_book_content(
    path: web::Path<(String, usize)>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let (book_id, content_idx) = path.into_inner();

    let content_path = app_state
        .lock()
        .unwrap()
        .voile
        .get_book_content_path(&book_id, content_idx)?;

    Ok(actix_files::NamedFile::open(content_path)?)
}

#[get("/user/book_progress/{book_id}")]
async fn get_book_progress(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    let book_progress = app_state
        .lock()
        .unwrap()
        .voile
        .get_book_progress(&book_id)?;

    Ok(web::Json(book_progress))
}

#[post("/user/book_progress/{book_id}")]
async fn set_book_progress(
    path: web::Path<String>,
    app_state: web::Data<SharedAppState>,
    book_progress: web::Json<crate::voile::voile::BookProgress>,
) -> actix_web::Result<impl Responder> {
    let book_id = path.into_inner();

    app_state
        .lock()
        .unwrap()
        .voile
        .set_book_progress(&book_id, &book_progress.0)?;

    Ok(book_progress)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books)
        .service(get_books_tags)
        .service(get_books_types)
        .service(get_books_langs)
        .service(get_book)
        .service(add_book)
        .service(delete_book)
        .service(get_book_cover)
        .service(set_book_cover)
        .service(set_book_detail)
        .service(get_book_content)
        .service(get_book_progress)
        .service(set_book_progress);
}
