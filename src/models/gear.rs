use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub uuid: Uuid,
    pub display_name: String,
    pub description: String,
    pub display_icon: Option<String>,
    pub asset_path: String,
    pub shop_data: GearShopData,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GearShopData {
    pub cost: u64,
    pub category: GearShopDataCategory,
    pub shop_order_priority: u8,
    pub category_text: String,
    // pub grid_position: // Currently always null
    pub can_be_trashed: bool,
    // pub image: // Currently always null
    pub new_image: String,
    // pub new_image2: // Currently always null
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum GearShopDataCategory {
    Armor,
}
