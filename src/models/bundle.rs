use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_name_sub_text: Option<String>,
    pub description: String,
    pub extra_description: Option<String>,
    pub promo_description: Option<String>,
    pub use_additional_context: bool,
    pub display_icon: String,
    pub display_icon2: String,
    pub asset_path: String,
}
