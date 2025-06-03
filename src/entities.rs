use std::fmt;

use anyhow::Result;

pub enum ReadState {
    NoChange,
    Change,
}

impl fmt::Display for ReadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadState::NoChange => write!(f, "NoChange"),
            ReadState::Change => write!(f, "Change"),
        }
    }
}

pub struct VersionInfo {
    pub state: ReadState,
    pub version_id: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Version {
    #[serde(rename = "versionId")]
    pub version_id: i64,
    #[serde(rename = "versionText")]
    pub version_text: String,
}

pub trait VersionAccess {
    /// return the latest version info or None if none exists
    async fn get_latest_version(&self) -> Result<Option<Version>>;
    async fn insert_version_text(&self, text: String) -> Result<i64>;
    async fn insert_info(&self, version_info: VersionInfo) -> Result<()>;
}

pub trait Notifier {
    fn message(&self, message: String) -> Result<()>;
}
