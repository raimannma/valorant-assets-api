use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Buddy {
    pub uuid: Uuid,
    pub display_name: String,
    pub is_hidden_if_not_owned: bool,
    pub theme_uuid: Option<Uuid>,
    pub display_icon: String,
    pub asset_path: String,
    pub levels: Vec<BuddyLevel>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuddyLevel {
    pub uuid: Uuid,
    pub charm_level: u16, // Always 1 currently
    pub display_name: String,
    pub hide_if_not_owned: bool,
    pub display_icon: String,
    pub asset_path: String,
}
