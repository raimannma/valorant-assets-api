use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub uuid: Uuid,
    pub display_name: String,
    pub r#type: Option<SeasonType>, // null if episode else SeasonType::Act
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub parent_uuid: Option<Uuid>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum SeasonType {
    #[serde(rename = "EAresSeasonType::Act")]
    Act,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompetitiveSeason {
    pub uuid: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub season_uuid: Uuid,
    pub competitive_tiers_uuid: Uuid,
    pub borders: Option<Vec<CompetitiveSeasonBorder>>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompetitiveSeasonBorder {
    pub uuid: Uuid,
    pub level: u8,
    pub wins_required: u32,
    pub display_icon: String,
    pub small_icon: Option<String>,
    pub asset_path: String,
}
