use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MrarError {
    #[error("IO error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Metadata strip failed on {path}: {source}")]
    Strip {
        path: PathBuf,
        #[source]
        source: metastrip::Error,
    },

    #[error("No supported images found in {0}")]
    NoImages(PathBuf),
}
