use aether_core::VersionId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    pub id: String,
    #[serde(default)]
    pub inherits_from: Option<String>,
    #[serde(default)]
    pub main_class: Option<String>,
}

impl VersionManifest {
    pub fn version_id(&self) -> Result<VersionId, ManifestError> {
        VersionId::parse(self.id.clone()).map_err(|_| ManifestError::InvalidVersionId)
    }
}

pub fn parse_version_manifest(input: &str) -> Result<VersionManifest, ManifestError> {
    serde_json::from_str(input).map_err(ManifestError::InvalidJson)
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("version manifest contains invalid JSON")]
    InvalidJson(#[source] serde_json::Error),
    #[error("version manifest contains an invalid version identifier")]
    InvalidVersionId,
}

#[cfg(test)]
mod tests {
    use super::parse_version_manifest;

    #[test]
    fn parses_minimal_manifest() {
        let manifest = parse_version_manifest(r#"{"id":"1.21.1"}"#).unwrap();
        assert_eq!(manifest.id, "1.21.1");
    }
}
