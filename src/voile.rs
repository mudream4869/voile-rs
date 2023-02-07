use std::str::FromStr;

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
}

impl BookDetails {
    pub fn new() -> BookDetails {
        BookDetails {
            title: None,
            author: None,
            tags: None,
            book_cover: None,
        }
    }

    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<BookDetails> {
        let detail_str = std::fs::read_to_string(filename)?;
        let detail: BookDetails = serde_json::from_str(detail_str.as_str())?;
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
    }
}

pub struct Voile {
    books_dir: String,

    book_cache: std::collections::HashMap<String, Book>,
    db_conn: std::sync::Mutex<sqlite::Connection>,
}

fn is_image(filename: &String) -> bool {
    // TODO
    let img_suffixes: Vec<&str> = vec![".png", ".jpg"];
    for suffix in img_suffixes {
        if filename.ends_with(suffix) {
            return true;
        }
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
                 content_idx INTEGER NOT NULL
             )",
        )?;

        Ok(())
    }

    pub fn get_book_proc(&self, book_id: String) -> Result<usize> {
        let db_conn = self.db_conn.lock().unwrap();
        let query = "SELECT content_idx FROM book_read_proc WHERE book_id = :book_id";
        let mut statement = db_conn.prepare(query)?;
        statement.bind::<&[(_, sqlite::Value)]>(&[(":book_id", book_id.clone().into())][..])?;

        let s = statement.next()?;

        if s == sqlite::State::Row {
            return Ok(statement.read::<i64, _>("content_idx").unwrap() as usize);
        }

        Err(Box::new(BookIDNotFoundError(book_id)))
    }

    pub fn set_book_proc(&self, book_id: String, content_idx: usize) -> Result<()> {
        let db_conn = self.db_conn.lock().unwrap();
        let query = r#"
            INSERT INTO book_read_proc (book_id, content_idx)
                VALUES (:book_id, :content_idx)
                ON CONFLICT (book_id) DO
                UPDATE SET content_idx = excluded.content_idx;
        "#;
        let mut statement = db_conn.prepare(query)?;

        statement.bind::<&[(_, sqlite::Value)]>(&[
            (":book_id", book_id.into()),
            (":content_idx", (content_idx as i64).into()),
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

        let mut contents = vec![];
        for path in std::fs::read_dir(full_dir)? {
            let entry = path?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            let title = String::from_str(entry.file_name().to_str().unwrap()).unwrap();

            if title == "details.json" {
                continue;
            } else if title.starts_with('.') {
                // hidden files
                continue;
            }

            contents.push(title);
        }

        contents.sort();

        let mut book = Book {
            book_id: book_id.clone(),
            title: book_id.clone(),
            content_titles: contents,
            author: None,
            tags: None,
            book_cover: None,
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

    pub fn get_book_content(
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
}
