use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_name_singular: String,
    pub display_icon: String,
    pub large_icon: String,
    pub asset_path: String,
}
