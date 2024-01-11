use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub store_featured_image: Option<String>,
    pub asset_path: String,
}
