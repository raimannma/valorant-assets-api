use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Spray {
    pub uuid: Uuid,
    pub display_name: String,
    pub category: Option<SprayCategory>,
    pub theme_uuid: Option<Uuid>,
    pub is_null_spray: bool,
    pub hide_if_not_owned: bool,
    pub display_icon: String,
    pub full_icon: Option<String>,
    pub animation_png: Option<String>,
    pub animation_gif: Option<String>,
    pub levels: Vec<SprayLevel>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SprayCategory {
    #[serde(rename = "EAresSprayCategory::Contextual")]
    Contextual,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SprayLevel {
    pub uuid: Uuid,
    pub spray_level: u8,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub asset_path: String,
}
