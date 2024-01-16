#[derive(Debug)]
pub struct BookIDNotFoundError(pub String);

impl std::fmt::Display for BookIDNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "book_id: {} not found", self.0)
    }
}

impl std::error::Error for BookIDNotFoundError {}

#[derive(Debug)]
pub struct IndexOutOfRange(pub usize);

impl std::fmt::Display for IndexOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "index: {} out of range", self.0)
    }
}

impl std::error::Error for IndexOutOfRange {}

#[derive(Debug)]
pub struct NotExist(pub String);

impl std::fmt::Display for NotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "data not exist: {}", self.0)
    }
}

impl std::error::Error for NotExist {}

#[derive(Debug)]
pub struct FileTypeError(pub String);

impl std::fmt::Display for FileTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "file type: {} not supported", self.0)
    }
}

impl std::error::Error for FileTypeError {}
