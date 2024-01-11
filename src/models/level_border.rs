use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LevelBorder {
    pub uuid: Uuid,
    pub display_name: String,
    pub starting_level: u16,
    pub level_number_appearance: String,
    pub small_player_card_appearance: String,
    pub asset_path: String,
}
