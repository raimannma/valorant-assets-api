use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub uuid: Uuid,
    pub display_name: String,
    pub description: String,
    pub developer_name: String,
    pub character_tags: Option<Vec<String>>,
    pub display_icon: String,
    pub display_icon_small: String,
    pub bust_portrait: Option<String>,
    pub full_portrait: Option<String>,
    pub full_portrait_v2: Option<String>,
    pub killfeed_portrait: Option<String>,
    pub background: Option<String>,
    pub background_gradient_colors: Vec<String>,
    pub asset_path: String,
    pub is_full_portrait_right_facing: bool,
    pub is_playable_character: bool,
    pub is_available_for_test: bool,
    pub is_base_content: bool,
    pub role: Option<AgentRole>,
    pub recruitment_data: Option<AgentRecruitmentData>,
    pub abilities: Vec<AgentAbility>,
    // pub voice_line:
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentRole {
    pub uuid: Uuid,
    pub display_name: String,
    pub description: String,
    pub display_icon: String,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentRecruitmentData {
    pub counter_id: Uuid,
    pub milestone_id: Uuid,
    pub milestone_threshold: u64,
    pub use_level_vp_cost_override: bool,
    pub level_vp_cost_override: i64, // Maybe u64?
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentAbility {
    pub slot: AgentAbilitySlot,
    pub display_name: String,
    pub description: String,
    pub display_icon: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum AgentAbilitySlot {
    Ability1,
    Ability2,
    Grenade,
    Ultimate,
    Passive,
}
