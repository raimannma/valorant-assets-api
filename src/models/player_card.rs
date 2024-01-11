use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCard {
    pub uuid: Uuid,
    pub display_name: String,
    pub is_hidden_if_not_owned: bool,
    pub theme_uuid: Option<Uuid>,
    pub display_icon: String,
    pub small_art: String,
    pub wide_art: String,
    pub large_art: String,
    pub asset_path: String,
}
