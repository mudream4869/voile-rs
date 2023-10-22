use std::path::Path;

use epub::doc::EpubDoc;
use pdf::file::FileOptions;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> std::io::Result<()> {
    if let Ok(()) = std::fs::rename(&source, &target) {
        return Ok(());
    }

    std::fs::copy(&source, &target)?;
    Ok(())
}

pub fn encode_to_utf8<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> Result<()> {
    let fp = std::fs::read(source)?;
    let res = chardet::detect(&fp);
    if let Some(coder) =
        encoding::label::encoding_from_whatwg_label(chardet::charset2encoding(&res.0))
    {
        let utf8reader = coder.decode(&fp, encoding::DecoderTrap::Ignore)?;
        std::fs::write(target, &utf8reader)?;
        return Ok(());
    }
    Err(Box::new(super::errors::FileTypeError(format!(
        "Failt to code to utf8: {:?}",
        res
    ))))
}

pub struct PDFMeta {
    pub title: Option<String>,
    pub author: Option<String>,
}

pub fn get_pdf_metadata<P: AsRef<Path>>(path: P) -> Result<PDFMeta> {
    let file = FileOptions::cached().open(&path)?;

    let mut meta = PDFMeta {
        title: None,
        author: None,
    };

    if let Some(info) = &file.trailer.info_dict {
        if let Some(author) = info.get("Author") {
            if let Ok(author) = author.to_string() {
                meta.author = Some(author);
            }
        }
        if let Some(title) = info.get("Title") {
            if let Ok(title) = title.to_string() {
                meta.title = Some(title);
            }
        }
    }

    Ok(meta)
}

pub struct EPubMeta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<Vec<u8>>,
}

pub fn get_epub_metadata<P: AsRef<Path>>(path: P) -> Result<EPubMeta> {
    let mut doc = EpubDoc::new(path)?;
    let title = doc.mdata("title");

    // TODO: support multiple authors
    let author = doc.mdata("creator");

    let mut meta = EPubMeta {
        title: title,
        author: author,
        cover: None,
    };

    if let Some((cover, _mime)) = doc.get_cover() {
        dbg!(_mime);
        meta.cover = Some(cover);
    }

    Ok(meta)
}
