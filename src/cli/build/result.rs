use thiserror::Error;

use super::target::BuildTarget;

/**
    Errors that may occur when building a standalone binary
*/
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("failed to find luneweb target '{0}' in GitHub release")]
    ReleaseTargetNotFound(BuildTarget),
    #[error("failed to find luneweb binary '{0}' in downloaded zip file")]
    ZippedBinaryNotFound(String),
    #[error("failed to download luneweb binary: {0}")]
    Download(#[from] reqwest::Error),
    #[error("failed to unzip luneweb binary: {0}")]
    Unzip(#[from] zip_next::result::ZipError),
    #[error("panicked while unzipping luneweb binary: {0}")]
    UnzipJoin(#[from] tokio::task::JoinError),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type BuildResult<T, E = BuildError> = std::result::Result<T, E>;
