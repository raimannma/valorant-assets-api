use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Ceremony {
    pub uuid: Uuid,
    pub display_name: String,
    pub asset_path: String,
}
