use std::path::Path;

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
