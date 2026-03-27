use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScalpelError {
    #[error("unsupported file type for path: {0}")]
    UnsupportedFileType(PathBuf),

    #[error("no matches found for pattern '{pattern}' in {path}")]
    NoMatch { pattern: String, path: PathBuf },

    #[error("no matches found for pattern '{pattern}'")]
    NoMatchFound { pattern: String },

    #[error("ambiguous pattern '{pattern}' in {path}: matched {count} symbols; use --index")]
    Ambiguous { pattern: String, path: PathBuf, count: usize },

    #[error("invalid rename directive, expected old=new")]
    InvalidRename,

    #[error("invalid replace directive, expected old=>new")]
    InvalidReplace,

    #[error("write failed for {path}: {message}")]
    WriteError { path: PathBuf, message: String },
}
