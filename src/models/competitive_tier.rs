use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompetitiveTiers {
    pub uuid: Uuid,
    pub asset_object_name: String,
    pub tiers: Vec<CompetitiveTier>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompetitiveTier {
    pub tier: u8,
    pub tier_name: String,
    pub division: CompetitiveTierDivision,
    pub division_name: String,
    pub color: String,
    pub background_color: String,
    pub small_icon: Option<String>,
    pub large_icon: Option<String>,
    pub rank_triangle_down_icon: Option<String>,
    pub rank_triangle_up_icon: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum CompetitiveTierDivision {
    #[serde(rename = "ECompetitiveDivision::UNRANKED")]
    Unranked,
    #[serde(rename = "ECompetitiveDivision::INVALID")]
    Invalid,
    #[serde(rename = "ECompetitiveDivision::IRON")]
    Iron,
    #[serde(rename = "ECompetitiveDivision::BRONZE")]
    Bronze,
    #[serde(rename = "ECompetitiveDivision::SILVER")]
    Silver,
    #[serde(rename = "ECompetitiveDivision::GOLD")]
    Gold,
    #[serde(rename = "ECompetitiveDivision::PLATINUM")]
    Platinum,
    #[serde(rename = "ECompetitiveDivision::DIAMOND")]
    Diamond,
    #[serde(rename = "ECompetitiveDivision::ASCENDANT")]
    Ascendant,
    #[serde(rename = "ECompetitiveDivision::IMMORTAL")]
    Immortal,
    #[serde(rename = "ECompetitiveDivision::RADIANT")]
    Radiant,
}
