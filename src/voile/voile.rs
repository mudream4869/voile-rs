use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub fn from_filename<P: AsRef<Path>>(filename: P) -> std::io::Result<BookDetails> {
        let detail_str = std::fs::read_to_string(filename)?;
        let detail: BookDetails = serde_json::from_str(&detail_str)?;
        Ok(detail)
    }

    pub fn write_to_filename<P: AsRef<Path>>(&self, filename: P) -> std::io::Result<()> {
        let detail_str = serde_json::to_string_pretty(&self)?;
        std::fs::write(filename, detail_str)?;
        Ok(())
    }
}

#[derive(Serialize, Clone, Debug)]
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
    pub fn apply_book_detail(&mut self, book_detail: &BookDetails) {
        if let Some(title) = book_detail.title.clone() {
            self.title = title;
        }

        if let Some(author) = book_detail.author.clone() {
            self.author = Some(author);
        }

        if let Some(tags) = book_detail.tags.clone() {
            self.tags = Some(tags);
        }

        if let Some(book_cover) = book_detail.book_cover.clone() {
            self.book_cover = Some(book_cover)
        }

        if let Some(book_type) = book_detail.book_type.clone() {
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

const DETAIL_FILENAME: &str = "details.json";
const BOOK_COVER_FILENAME: &str = "book_cover.jpg";

pub struct Voile {
    books_dir: String,

    book_cache: std::collections::HashMap<String, Book>,
    db_conn: std::sync::Mutex<sqlite::Connection>,

    book_id_retry_number: i32,
}

fn is_image(file_path: &str) -> bool {
    let path = Path::new(file_path);
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
            book_id_retry_number: 100,
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

    pub fn get_book_proc(&self, book_id: &str) -> Result<BookProc> {
        let db_conn = self.db_conn.lock().unwrap();
        let query = "SELECT content_idx, paging FROM book_read_proc WHERE book_id = :book_id";
        let mut statement = db_conn.prepare(query)?;
        statement.bind::<&[(_, sqlite::Value)]>(&[(":book_id", book_id.into())][..])?;

        let s = statement.next()?;

        if s == sqlite::State::Row {
            return Ok(BookProc {
                content_idx: statement.read::<i64, _>("content_idx").unwrap() as usize,
                paging: statement.read::<i64, _>("paging").unwrap() as usize,
            });
        }

        Err(Box::new(super::errors::BookIDNotFoundError(
            book_id.to_string(),
        )))
    }

    pub fn set_book_proc(&self, book_id: &str, book_proc: &BookProc) -> Result<()> {
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
        for path in std::fs::read_dir(&self.books_dir)? {
            let entry = path?;

            if !entry.file_type()?.is_dir() {
                continue;
            }

            let book_id = if let Some(filename) = entry.file_name().to_str() {
                filename.to_string()
            } else {
                log::info!("Fail to get filename: {:?}", entry);
                continue;
            };

            if book_id.starts_with(".") {
                // hidden files
                continue;
            }

            match self.get_book(&book_id) {
                Ok(book) => ret.push(book),
                Err(_) => {}
            }
        }
        Ok(ret)
    }

    fn get_book_dir(&self, book_id: &str) -> PathBuf {
        [&self.books_dir, book_id].iter().collect()
    }

    pub fn get_book(&mut self, book_id: &str) -> Result<Book> {
        if let Some(book) = self.book_cache.get(book_id) {
            return Ok(book.clone());
        }

        // TODO: dir safety check
        let book_dir = self.get_book_dir(book_id);

        let default_created_time = std::fs::metadata(&book_dir)?
            .created()?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let mut default_modified_time = default_created_time;
        let mut default_book_cover: Option<String> = None;

        let mut content_titles = vec![];
        for path in std::fs::read_dir(&book_dir)? {
            let entry = path?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            let filename = entry.file_name().to_str().unwrap().to_string();

            if filename == DETAIL_FILENAME {
                default_modified_time = entry
                    .metadata()?
                    .modified()?
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                    .as_secs();
                continue;
            } else if filename == BOOK_COVER_FILENAME {
                default_book_cover = Some(filename);
                continue;
            } else if filename.starts_with('.') {
                // hidden files
                continue;
            }

            content_titles.push(filename);
        }

        content_titles.sort();

        let local_path = Path::new(&book_dir).absolutize().unwrap();

        let mut book = Book {
            book_id: book_id.to_string(),
            title: book_id.to_string(),
            content_titles,
            author: None,
            tags: None,
            book_cover: None,
            book_type: None,

            created_timestamp: default_created_time,
            modified_timestamp: default_modified_time,
            local_path: local_path.to_str().unwrap().to_string(),
        };

        // details.json is optional
        let detail_filename = book_dir.join(DETAIL_FILENAME);

        if let Ok(book_detail) = BookDetails::from_filename(detail_filename) {
            book.apply_book_detail(&book_detail);
        }

        if book.book_cover.is_none() {
            if let Some(filename) = &default_book_cover {
                if is_image(filename) {
                    book.book_cover = Some(filename.clone());
                }
            }

            // Use first content as book_cover if it's an image file
            if let Some(filename) = book.content_titles.get(0) {
                if is_image(filename) {
                    book.book_cover = Some(filename.clone());
                }
            }
        }

        self.book_cache.insert(book_id.to_string(), book.clone());
        Ok(book)
    }

    pub fn delete_book(&mut self, book_id: &str) -> Result<()> {
        if self.book_cache.get(book_id).is_none() {
            return Ok(());
        }

        std::fs::remove_dir_all(self.get_book_dir(book_id))?;

        self.book_cache.remove(book_id);

        Ok(())
    }

    pub async fn add_book(&self, filename: &str, filesource: PathBuf) -> Result<()> {
        if let Some(book_id) = filename.strip_suffix(".txt") {
            self.add_book_txt(filesource, filename, book_id).await?;
            return Ok(());
        } else if let Some(book_id) = filename.strip_suffix(".pdf") {
            self.add_book_pdf(filesource, filename, book_id).await?;
            return Ok(());
        } else if let Some(book_id) = filename.strip_suffix(".zip") {
            self.add_book_zip(filesource, book_id).await?;
            return Ok(());
        }

        Err(Box::new(super::errors::FileTypeError(
            "Not match txt, pdf or zip".to_string(),
        )))
    }

    fn create_valid_book_id(&self, base_book_id: &str) -> Result<String> {
        {
            let folderpath = self.get_book_dir(base_book_id);
            if std::fs::create_dir(&folderpath).is_ok() {
                return Ok(base_book_id.to_string());
            }
        }

        for i in 0..self.book_id_retry_number {
            let try_book_id: String = format!("{}_{}", base_book_id, i);
            let folderpath = self.get_book_dir(&try_book_id);
            if std::fs::create_dir(&folderpath).is_ok() {
                return Ok(try_book_id.to_string());
            }
        }

        Err(Box::new(super::errors::NotExist(
            "valid book id".to_string(),
        )))
    }

    async fn add_book_txt(&self, filesource: PathBuf, filename: &str, book_id: &str) -> Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let tmp_filename = tmp_dir.path().join("tmp");
        super::util::encode_to_utf8(filesource, &tmp_filename)?;

        let valid_book_id = self.create_valid_book_id(book_id)?;
        let folderpath = self.get_book_dir(&valid_book_id);

        let filepath = folderpath.join(filename);

        crate::voile::util::move_file(tmp_filename, filepath)?;
        Ok(())
    }

    async fn add_book_pdf(&self, filesource: PathBuf, filename: &str, book_id: &str) -> Result<()> {
        let valid_book_id = self.create_valid_book_id(book_id)?;
        let folderpath = self.get_book_dir(&valid_book_id);

        let filepath = folderpath.join(filename);

        crate::voile::util::move_file(filesource, filepath)?;
        Ok(())
    }

    async fn add_book_zip(&self, filesource: PathBuf, book_id: &str) -> Result<()> {
        let valid_book_id = self.create_valid_book_id(book_id)?;
        let folderpath = self.get_book_dir(&valid_book_id);

        // TODO: exception safe

        let zip_file = std::fs::File::open(filesource.clone())?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let filename = file.name();
            if filename.ends_with('/') {
                continue;
            }

            let out_filepath = folderpath.join(filename);

            let mut outfile = std::fs::File::create(&out_filepath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        Ok(())
    }

    pub fn get_book_content_path(&mut self, book_id: &str, content_idx: usize) -> Result<PathBuf> {
        // TODO: dir safety check
        let book = self.get_book(book_id)?;
        let content_id = match book.content_titles.get(content_idx) {
            Some(s) => s,
            None => return Err(Box::new(super::errors::IndexOutOfRange(content_idx))),
        };

        Ok(self.get_book_dir(&book_id).join(content_id))
    }

    pub fn get_book_cover_path(&mut self, book_id: &str) -> Result<PathBuf> {
        // TODO: dir safety check
        let book = self.get_book(book_id)?;

        let book_cover = if let Some(book_cover) = book.book_cover {
            book_cover
        } else {
            return Err(Box::new(super::errors::NotExist("book_cover".to_string())));
        };

        Ok(self.get_book_dir(&book_id).join(book_cover))
    }

    pub async fn set_book_cover(&mut self, book_id: &str, filesource: PathBuf) -> Result<()> {
        let filepath = self.get_book_dir(book_id).join(BOOK_COVER_FILENAME);

        crate::voile::util::move_file(filesource, filepath)?;
        self.book_cache.remove(book_id);

        Ok(())
    }

    pub fn set_book_detail(&mut self, book_id: &str, book_detail: BookDetails) -> Result<()> {
        self.get_book(book_id)?;

        let book_id_string = book_id.to_string();
        let cur_book = self.book_cache.get_mut(&book_id_string).unwrap();
        cur_book.apply_book_detail(&book_detail);

        let detail_filename = self.get_book_dir(book_id).join(DETAIL_FILENAME);

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
