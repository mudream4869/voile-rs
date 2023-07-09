use std::path::Path;

pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> std::io::Result<()> {
    if let Ok(()) = std::fs::rename(&source, &target) {
        return Ok(());
    }

    std::fs::copy(&source, &target)?;
    Ok(())
}
