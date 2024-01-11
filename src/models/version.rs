use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub manifest_id: String,
    pub branch: String,
    pub version: String,
    pub build_version: String,
    pub engine_version: String,
    pub riot_client_version: String,
    pub riot_client_build: String,
    pub build_date: DateTime<Utc>,
}
