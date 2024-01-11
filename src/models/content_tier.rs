use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentTier {
    pub uuid: Uuid,
    pub display_name: String,
    pub dev_name: String,
    pub rank: u16,
    pub juice_value: u16,
    pub juice_cost: u16,
    pub highlight_color: String,
    pub display_icon: String,
    pub asset_path: String,
}
