use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSource {
    pub provider: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadArtifact {
    pub destination: PathBuf,
    pub size: u64,
    pub sha1: String,
    pub sources: Vec<ArtifactSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DownloadPlan {
    pub artifacts: Vec<DownloadArtifact>,
}

impl DownloadPlan {
    pub fn validate(&self) -> Result<(), DownloadPlanError> {
        for artifact in &self.artifacts {
            if artifact.sources.is_empty() {
                return Err(DownloadPlanError::NoSource(artifact.destination.clone()));
            }
            if artifact.sha1.len() != 40
                || !artifact
                    .sha1
                    .chars()
                    .all(|character| character.is_ascii_hexdigit())
            {
                return Err(DownloadPlanError::InvalidSha1(artifact.destination.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DownloadPlanError {
    #[error("artifact {0:?} has no download source")]
    NoSource(PathBuf),
    #[error("artifact {0:?} has an invalid SHA-1 digest")]
    InvalidSha1(PathBuf),
}
