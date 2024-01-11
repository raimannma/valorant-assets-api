use crate::models::weapon::WeaponCategory;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Gamemode {
    pub uuid: Uuid,
    pub display_name: String,
    pub duration: Option<String>,
    pub economy_type: Option<GamemodeEconomyType>,
    pub allows_match_timeouts: bool,
    pub is_team_voice_allowed: bool,
    pub is_minimap_hidden: bool,
    pub orb_count: u8,
    pub rounds_per_half: i8,
    pub team_roles: Option<Vec<GamemodeTeamRole>>,
    pub game_feature_overrides: Option<Vec<GamemodeGameFeatureOverride>>,
    pub game_rule_bool_overrides: Option<Vec<GamemodeGameRuleBoolOverride>>,
    pub display_icon: Option<String>,
    pub list_view_icon_tall: Option<String>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GamemodeEquippable {
    pub uuid: Uuid,
    pub display_name: String,
    pub category: WeaponCategory,
    pub display_icon: String,
    pub kill_stream_icon: String,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum GamemodeEconomyType {
    #[serde(rename = "EEconomyTypes::Other")]
    Other,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum GamemodeTeamRole {
    #[serde(rename = "EAresTeamRole::FreeForAll")]
    FreeForAll,
    #[serde(rename = "EAresTeamRole::Attacker")]
    Attacker,
    #[serde(rename = "EAresTeamRole::Defender")]
    Defender,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GamemodeGameFeatureOverride {
    pub feature_name: GamemodeGameFeatureOverrideFeature,
    pub state: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum GamemodeGameFeatureOverrideFeature {
    #[serde(rename = "EGameFeatureToggleName::AllowShoppingWhileDead")]
    AllowShoppingWhileDead,
    #[serde(rename = "EGameFeatureToggleName::DeathmatchEncourageFarSpawning")]
    DeathmatchEncourageFarSpawning,
    #[serde(rename = "EGameFeatureToggleName::DisableFogOfWar")]
    DisableFogOfWar,
    #[serde(rename = "EGameFeatureToggleName::EquippableCacheRecycling")]
    EquippableCacheRecycling,
    #[serde(rename = "EGameFeatureToggleName::ReuseActorOnRespawn")]
    ReuseActorOnRespawn,
    #[serde(rename = "EGameFeatureToggleName::UseMeshMaterialManager")]
    UseMeshMaterialManager,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GamemodeGameRuleBoolOverride {
    pub rule_name: GamemodeGameRuleBoolOverrideRule,
    pub state: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum GamemodeGameRuleBoolOverrideRule {
    #[serde(rename = "EGameRuleBoolName::AssignRandomAgents")]
    AssignRandomAgents,
    #[serde(rename = "EGameRuleBoolName::CombatReportOnlyShowLastLife")]
    CombatReportOnlyShowLastLife,
    #[serde(rename = "EGameRuleBoolName::DestroyAbilitiesOnDeath")]
    DestroyAbilitiesOnDeath,
    #[serde(rename = "EGameRuleBoolName::DownedCharactersCanGiveUp")]
    DownedCharactersCanGiveUp,
    #[serde(rename = "EGameRuleBoolName::IsOvertimeWinByTwo")]
    IsOvertimeWinByTwo,
    #[serde(rename = "EGameRuleBoolName::MajorityVoteAgents")]
    MajorityVoteAgents,
    #[serde(rename = "EGameRuleBoolName::PipAbilityCasting")]
    PipAbilityCasting,
    #[serde(rename = "EGameRuleBoolName::PreventAbilityRecharge")]
    PreventAbilityRecharge,
    #[serde(rename = "EGameRuleBoolName::SkipPregame")]
    SkipPregame,
    #[serde(rename = "EGameRuleBoolName::UseAllAbilityCooldowns")]
    UseAllAbilityCooldowns,
    #[serde(rename = "EGameRuleBoolName::UseInDevWeapons")]
    UseInDevWeapons,
}
