use std::io::Write;

use futures_util::TryStreamExt;

pub async fn download_file_from_multipart<P: AsRef<std::path::Path>>(
    mut field: actix_multipart::Field,
    filepath: P,
) -> actix_web::Result<()> {
    let mut f = std::fs::File::create(filepath)?;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.try_next().await? {
        // filesystem operations are blocking, we have to use threadpool
        f = actix_web::web::block(move || f.write_all(&chunk).map(|_| f)).await??;
    }

    Ok(())
}
