use std::path::Path;

use anyhow::Context;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn read_file_streamed(path: &Path, max_bytes: u64) -> anyhow::Result<String> {
    let meta =
        tokio::fs::metadata(path).await.with_context(|| format!("metadata {}", path.display()))?;
    if meta.len() > max_bytes {
        anyhow::bail!("file exceeds max size: {}", path.display());
    }

    let file =
        tokio::fs::File::open(path).await.with_context(|| format!("opening {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader
            .read_line(&mut line)
            .await
            .with_context(|| format!("reading {}", path.display()))?;
        if bytes == 0 {
            break;
        }
        content.push_str(&line);
    }

    Ok(content)
}
