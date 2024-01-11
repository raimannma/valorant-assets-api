use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTitle {
    pub uuid: Uuid,
    pub display_name: Option<String>,
    pub title_text: Option<String>,
    pub is_hidden_if_not_owned: bool,
    pub asset_path: String,
}
