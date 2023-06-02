use std::io::Write;
use std::str::FromStr;

use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct BookIDNotFoundError(String);

impl std::fmt::Display for BookIDNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "book_id: {} not found", self.0)
    }
}

impl std::error::Error for BookIDNotFoundError {}

#[derive(Debug)]
struct IndexOutOfRange(usize);

impl std::fmt::Display for IndexOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "index: {} out of range", self.0)
    }
}

impl std::error::Error for IndexOutOfRange {}

#[derive(Debug)]
struct NotExist(String);

impl std::fmt::Display for NotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "data not exist: {}", self.0)
    }
}

impl std::error::Error for NotExist {}

#[derive(Debug)]
struct FileTypeError(String);

impl std::fmt::Display for FileTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "file type: {} not supported", self.0)
    }
}

impl std::error::Error for FileTypeError {}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_cover: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_type: Option<String>,
}

impl BookDetails {
    pub fn new() -> BookDetails {
        BookDetails {
            title: None,
            author: None,
            tags: None,
            book_cover: None,
            book_type: None,
        }
    }

    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<BookDetails> {
        let detail_str = std::fs::read_to_string(filename)?;
        let detail: BookDetails = serde_json::from_str(&detail_str)?;
        Ok(detail)
    }

    pub fn write_to_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> std::io::Result<()> {
        let detail_str = serde_json::to_string_pretty(&self)?;
        std::fs::write(filename, detail_str)?;
        Ok(())
    }
}

#[derive(Serialize, Clone)]
pub struct Book {
    pub book_id: String,
    pub title: String,

    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub content_titles: Vec<String>,
    pub book_cover: Option<String>,
    pub book_type: Option<String>,

    // Fixed info
    pub created_timestamp: u64,
    pub modified_timestamp: u64,
    pub local_path: String,
}

impl Book {
    pub fn apply_book_detail(&mut self, book_detail: BookDetails) {
        if let Some(title) = book_detail.title {
            self.title = title;
        }

        if let Some(author) = book_detail.author {
            self.author = Some(author);
        }

        if let Some(tags) = book_detail.tags {
            self.tags = Some(tags);
        }

        if let Some(book_cover) = book_detail.book_cover {
            self.book_cover = Some(book_cover)
        }

        if let Some(book_type) = book_detail.book_type {
            self.book_type = Some(book_type)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookProc {
    pub content_idx: usize,
    pub paging: usize,
}

impl BookProc {
    pub fn new() -> BookProc {
        BookProc {
            content_idx: 0,
            paging: 0,
        }
    }
}

pub struct Voile {
    books_dir: String,

    book_cache: std::collections::HashMap<String, Book>,
    db_conn: std::sync::Mutex<sqlite::Connection>,
}

fn is_image(file_path: &str) -> bool {
    let path = std::path::Path::new(file_path);
    let extension = path.extension();

    if let Some(ext) = extension {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" || ext_str == "gif";
    }

    false
}

impl Voile {
    pub fn new(books_dir: String) -> Result<Self> {
        let db_conn = sqlite::open(":memory:")?;

        let mut ret = Self {
            books_dir: books_dir,
            book_cache: std::collections::HashMap::new(),
            db_conn: std::sync::Mutex::new(db_conn),
        };

        ret.init()?;
        Ok(ret)
    }

    pub fn init(&mut self) -> Result<()> {
        let db_conn = self.db_conn.lock().unwrap();
        db_conn.execute(
            "CREATE TABLE IF NOT EXISTS book_read_proc (
                 book_id TEXT NOT NULL UNIQUE,
                 content_idx INTEGER NOT NULL,
                 paging INTEGER NOT NULL
             )",
        )?;

        Ok(())
    }

    pub fn get_book_proc(&self, book_id: String) -> Result<BookProc> {
        let db_conn = self.db_conn.lock().unwrap();
        let query = "SELECT content_idx, paging FROM book_read_proc WHERE book_id = :book_id";
        let mut statement = db_conn.prepare(query)?;
        statement.bind::<&[(_, sqlite::Value)]>(&[(":book_id", book_id.clone().into())][..])?;

        let s = statement.next()?;

        if s == sqlite::State::Row {
            return Ok(BookProc {
                content_idx: statement.read::<i64, _>("content_idx").unwrap() as usize,
                paging: statement.read::<i64, _>("paging").unwrap() as usize,
            });
        }

        Err(Box::new(BookIDNotFoundError(book_id)))
    }

    pub fn set_book_proc(&self, book_id: String, book_proc: &BookProc) -> Result<()> {
        let db_conn = self.db_conn.lock().unwrap();
        let query = r#"
            INSERT INTO book_read_proc (book_id, content_idx, paging)
                VALUES (:book_id, :content_idx, :paging)
                ON CONFLICT (book_id) DO
                UPDATE SET content_idx = excluded.content_idx, paging = excluded.paging;
        "#;
        let mut statement = db_conn.prepare(query)?;

        statement.bind::<&[(_, sqlite::Value)]>(&[
            (":book_id", book_id.into()),
            (":content_idx", (book_proc.content_idx as i64).into()),
            (":paging", (book_proc.paging as i64).into()),
        ])?;

        statement.next()?;

        Ok(())
    }

    pub fn get_books(&mut self) -> Result<Vec<Book>> {
        let mut ret = vec![];
        for path in std::fs::read_dir(self.books_dir.as_str())? {
            let entry = path?;

            if !entry.file_type()?.is_dir() {
                continue;
            }

            let title = String::from_str(entry.file_name().to_str().unwrap()).unwrap();

            if title.starts_with(".") {
                // hidden files
                continue;
            }

            match self.get_book(title) {
                Ok(book) => ret.push(book),
                Err(_) => {}
            }
        }
        Ok(ret)
    }

    pub fn get_book(&mut self, book_id: String) -> Result<Book> {
        if let Some(book) = self.book_cache.get(&book_id) {
            return Ok(book.clone());
        }

        // TODO: dir safety check
        let full_dir: std::path::PathBuf =
            [self.books_dir.as_str(), book_id.as_str()].iter().collect();

        let default_created_time = std::fs::metadata(full_dir.clone())?
            .created()?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut default_modified_time = default_created_time;

        let mut content_titles = vec![];
        for path in std::fs::read_dir(full_dir.clone())? {
            let entry = path?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            let title = String::from_str(entry.file_name().to_str().unwrap())?;

            if title == "details.json" {
                default_modified_time = entry
                    .metadata()?
                    .modified()?
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                    .as_secs();
                continue;
            } else if title.starts_with('.') {
                // hidden files
                continue;
            }

            content_titles.push(title);
        }

        content_titles.sort();

        let local_path = std::fs::canonicalize(full_dir.clone())?;

        let mut book = Book {
            book_id: book_id.clone(),
            title: book_id.clone(),
            content_titles,
            author: None,
            tags: None,
            book_cover: None,
            book_type: None,

            created_timestamp: default_created_time,
            modified_timestamp: default_modified_time,
            local_path: String::from(local_path.to_string_lossy()),
        };

        // details.json is optional
        let detail_filename: std::path::PathBuf =
            [self.books_dir.as_str(), book_id.as_str(), "details.json"]
                .iter()
                .collect();

        if let Ok(book_detail) = BookDetails::from_filename(detail_filename) {
            book.apply_book_detail(book_detail);
        }

        if book.book_cover.is_none() {
            // Use first content as book_cover if it's an image file
            if let Some(filename) = book.content_titles.get(0) {
                if is_image(filename) {
                    book.book_cover = Some(filename.clone());
                }
            }
        }

        self.book_cache.insert(book_id.clone(), book.clone());
        Ok(book)
    }

    pub fn delete_book(&mut self, book_id: String) -> Result<()> {
        if self.book_cache.get(&book_id).is_none() {
            return Ok(());
        }

        let full_dir: std::path::PathBuf =
            [self.books_dir.as_str(), book_id.as_str()].iter().collect();

        std::fs::remove_dir_all(full_dir)?;

        self.book_cache.remove(&book_id);

        Ok(())
    }

    pub async fn add_book(&self, field: actix_multipart::Field) -> Result<()> {
        // TODO: refine error

        let filename = field.content_disposition().get_filename().unwrap();

        if filename.ends_with(".txt") {
            return self.add_book_txt(field).await;
        } else if filename.ends_with(".zip") {
            return self.add_book_zip(field).await;
        }

        Err(Box::new(FileTypeError(
            String::from_str("Not match txt or zip").unwrap(),
        )))
    }

    async fn download_file_from_multipart(
        &self,
        mut field: actix_multipart::Field,
        filepath: std::path::PathBuf,
    ) -> Result<()> {
        let mut f = actix_web::web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = actix_web::web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }

        Ok(())
    }

    async fn add_book_txt(&self, field: actix_multipart::Field) -> Result<()> {
        let filename = field.content_disposition().get_filename().unwrap();

        let book_id = filename.strip_suffix(".txt").unwrap();

        let folderpath: std::path::PathBuf = [self.books_dir.as_str(), book_id].iter().collect();

        // prevent same folder_name
        std::fs::create_dir(folderpath)?;

        let filepath: std::path::PathBuf = [self.books_dir.as_str(), book_id, filename]
            .iter()
            .collect();

        self.download_file_from_multipart(field, filepath).await
    }

    async fn add_book_zip(&self, field: actix_multipart::Field) -> Result<()> {
        let filename = String::from_str(field.content_disposition().get_filename().unwrap())?;

        let book_id = filename.strip_suffix(".zip").unwrap();

        let folderpath: std::path::PathBuf = [self.books_dir.as_str(), book_id].iter().collect();

        // TODO: exception safe
        // prevent same folder_name
        std::fs::create_dir(folderpath)?;

        let tmp_dir = tempfile::tempdir()?;

        let zip_filepath = tmp_dir.path().join(filename.as_str());

        self.download_file_from_multipart(field, zip_filepath.clone())
            .await?;

        let zip_file = std::fs::File::open(zip_filepath.clone())?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let filename = file.name();
            if filename.ends_with('/') {
                continue;
            }

            let out_filepath: std::path::PathBuf = [self.books_dir.as_str(), book_id, filename]
                .iter()
                .collect();

            let mut outfile = std::fs::File::create(&out_filepath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        Ok(())
    }

    pub fn get_book_content_path(
        &mut self,
        book_id: String,
        content_idx: usize,
    ) -> Result<std::path::PathBuf> {
        // TODO: dir safety check
        let book = self.get_book(book_id.clone())?;
        let content_id = match book.content_titles.get(content_idx) {
            Some(s) => s,
            None => return Err(Box::new(IndexOutOfRange(content_idx))),
        };

        let full_dir: std::path::PathBuf = [
            self.books_dir.as_str(),
            book_id.as_str(),
            content_id.as_str(),
        ]
        .iter()
        .collect();

        Ok(full_dir)
    }

    pub fn get_book_cover_path(&mut self, book_id: String) -> Result<std::path::PathBuf> {
        // TODO: dir safety check
        let book = self.get_book(book_id.clone())?;
        if book.book_cover.is_none() {
            return Err(Box::new(NotExist(String::from_str("book_cover").unwrap())));
        }

        let book_cover = book.book_cover.unwrap();

        let full_dir: std::path::PathBuf = [
            self.books_dir.as_str(),
            book_id.as_str(),
            book_cover.as_str(),
        ]
        .iter()
        .collect();

        Ok(full_dir)
    }

    pub fn set_book_detail(&mut self, book_id: String, book_detail: BookDetails) -> Result<()> {
        self.get_book(book_id.clone())?;

        let cur_book = self.book_cache.get_mut(&book_id).unwrap();
        cur_book.apply_book_detail(book_detail.clone());

        let detail_filename: std::path::PathBuf =
            [self.books_dir.as_str(), book_id.as_str(), "details.json"]
                .iter()
                .collect();

        book_detail.write_to_filename(detail_filename)?;
        Ok(())
    }

    pub fn get_all_book_types(&mut self) -> Result<Vec<String>> {
        let mut ret = vec![];
        for book in self.get_books()? {
            if let Some(book_type) = book.book_type {
                ret.push(book_type)
            }
        }
        ret.sort();
        ret.dedup();
        Ok(ret)
    }

    pub fn get_all_book_tags(&mut self) -> Result<Vec<String>> {
        let mut ret = vec![];
        for book in self.get_books()? {
            if let Some(tags) = book.tags {
                ret.extend(tags);
            }
        }
        ret.sort();
        ret.dedup();
        Ok(ret)
    }
}
