use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub ship_it: bool,
    #[serde(rename = "useLevelVPCostOverride")]
    pub use_level_vp_cost_override: bool,
    #[serde(rename = "levelVPCostOverride")]
    pub level_vp_cost_override: i64,
    pub free_reward_schedule_uuid: Uuid,
    pub content: ContractContent,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractContent {
    pub relation_type: Option<ContractContentRelationType>,
    pub relation_uuid: Option<Uuid>,
    pub chapters: Vec<ContractContentChapter>,
    pub premium_reward_schedule_uuid: Option<Uuid>,
    #[serde(rename = "premiumVPCost")]
    pub premium_vp_cost: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ContractContentRelationType {
    Agent,
    Event,
    Season,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractContentChapter {
    pub is_epilogue: bool,
    pub levels: Vec<ContractContentChapterLevel>,
    pub free_rewards: Option<Vec<ContractContentChapterReward>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractContentChapterLevel {
    pub reward: ContractContentChapterReward,
    pub xp: u64,
    pub vp_cost: u64,
    #[serde(rename = "isPurchasableWithVP")]
    pub is_purchasable_with_vp: bool,
    pub dough_cost: u64,
    pub is_purchasable_with_dough: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractContentChapterReward {
    pub r#type: ContractContentChapterRewardType,
    pub uuid: Uuid,
    pub amount: u64,
    pub is_highlighted: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ContractContentChapterRewardType {
    Currency,
    EquippableCharmLevel,
    EquippableSkinLevel,
    PlayerCard,
    Spray,
    Title,
}
