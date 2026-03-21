use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use tokio::io::AsyncWriteExt;

use crate::error::ScalpelError;

pub struct Transaction {
    snapshot_dir: PathBuf,
    backups: HashMap<PathBuf, PathBuf>,
}

impl Transaction {
    pub async fn begin(paths: &[PathBuf]) -> anyhow::Result<Self> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("system clock before unix epoch")?
            .as_nanos();
        let snapshot_dir = std::env::temp_dir().join(format!("scalpel-snapshot-{stamp}"));
        tokio::fs::create_dir_all(&snapshot_dir).await?;

        let mut backups = HashMap::new();
        for path in paths {
            let file_name = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            let backup = snapshot_dir.join(file_name);
            tokio::fs::copy(path, &backup)
                .await
                .with_context(|| format!("snapshot copy {}", path.display()))?;
            backups.insert(path.clone(), backup);
        }

        Ok(Self { snapshot_dir, backups })
    }

    pub async fn atomic_write(path: &Path, new_content: &str) -> anyhow::Result<()> {
        let parent = path.parent().context("target path has no parent")?;
        let stem = path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".to_string());
        let temp = parent.join(format!(".{stem}.scalpel.tmp"));

        let file = tokio::fs::File::create(&temp)
            .await
            .with_context(|| format!("create temp {}", temp.display()))?;
        let mut writer = tokio::io::BufWriter::new(file);
        writer
            .write_all(new_content.as_bytes())
            .await
            .with_context(|| format!("write temp {}", temp.display()))?;
        writer.flush().await.with_context(|| format!("flush temp {}", temp.display()))?;
        drop(writer);

        tokio::fs::rename(&temp, path).await.map_err(|e| ScalpelError::WriteError {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        Ok(())
    }

    pub async fn rollback(&self) -> anyhow::Result<()> {
        for (target, backup) in &self.backups {
            tokio::fs::copy(backup, target)
                .await
                .with_context(|| format!("rollback {}", target.display()))?;
        }
        Ok(())
    }

    pub async fn cleanup(&self) -> anyhow::Result<()> {
        if self.snapshot_dir.exists() {
            tokio::fs::remove_dir_all(&self.snapshot_dir).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Transaction;

    #[tokio::test]
    async fn happy_path_atomic_write_updates_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let file = temp.path().join("a.txt");
        std::fs::write(&file, "old").expect("seed file");

        Transaction::atomic_write(&file, "new").await.expect("write");
        let content = std::fs::read_to_string(file).expect("read file");
        assert_eq!(content, "new");
    }

    #[tokio::test]
    async fn critical_path_rollback_restores_original() {
        let temp = tempfile::tempdir().expect("tempdir");
        let file = temp.path().join("a.txt");
        std::fs::write(&file, "seed").expect("seed file");

        let tx = Transaction::begin(std::slice::from_ref(&file)).await.expect("begin tx");
        Transaction::atomic_write(&file, "modified").await.expect("write modified");
        tx.rollback().await.expect("rollback");
        tx.cleanup().await.expect("cleanup");

        let content = std::fs::read_to_string(file).expect("read file");
        assert_eq!(content, "seed");
    }
}
