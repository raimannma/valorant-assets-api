use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub uuid: Uuid,
    pub display_name: String,
    pub category: WeaponCategory,
    pub default_skin_uuid: Uuid,
    pub display_icon: Option<String>,
    pub kill_stream_icon: String,
    pub asset_path: String,
    pub weapon_stats: Option<WeaponStats>,
    pub shop_data: Option<WeaponShopData>,
    pub skins: Vec<WeaponSkin>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponCategory {
    #[serde(rename = "EEquippableCategory::Heavy")]
    Heavy,
    #[serde(rename = "EEquippableCategory::Melee")]
    Melee,
    #[serde(rename = "EEquippableCategory::Rifle")]
    Rifle,
    #[serde(rename = "EEquippableCategory::Shotgun")]
    Shotgun,
    #[serde(rename = "EEquippableCategory::Sidearm")]
    Sidearm,
    #[serde(rename = "EEquippableCategory::SMG")]
    SMG,
    #[serde(rename = "EEquippableCategory::Sniper")]
    Sniper,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponStats {
    pub fire_rate: f64,
    pub magazine_size: i64,
    pub run_speed_multiplier: f64,
    pub equip_time_seconds: f64,
    pub reload_time_seconds: f64,
    pub first_bullet_accuracy: f64,
    pub shotgun_pellet_count: i64,
    pub wall_penetration: Option<WeaponStatsWallPenetration>,
    pub feature: Option<WeaponStatsFeature>,
    pub fire_mode: Option<WeaponStatsFireMode>,
    pub alt_fire_type: Option<WeaponStatsAltFireMode>,
    pub ads_stats: Option<WeaponStatsADSStats>,
    pub alt_shotgun_stats: Option<WeaponStatsAltShotgunStats>,
    pub air_burst_stats: Option<WeaponStatsAirBurstStats>,
    pub damage_ranges: Vec<WeaponStatsDamageRange>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponStatsWallPenetration {
    #[serde(rename = "EWallPenetrationDisplayType::Low")]
    Low,
    #[serde(rename = "EWallPenetrationDisplayType::Medium")]
    Medium,
    #[serde(rename = "EWallPenetrationDisplayType::High")]
    High,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponStatsFeature {
    #[serde(rename = "EWeaponStatsFeature::DualZoom")]
    DualZoom,
    #[serde(rename = "EWeaponStatsFeature::ROFIncrease")]
    ROFIncrease,
    #[serde(rename = "EWeaponStatsFeature::Silenced")]
    Silenced,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponStatsFireMode {
    #[serde(rename = "EWeaponFireModeDisplayType::SemiAutomatic")]
    SemiAutomatic,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponStatsAltFireMode {
    #[serde(rename = "EWeaponAltFireDisplayType::ADS")]
    ADS,
    #[serde(rename = "EWeaponAltFireDisplayType::AirBurst")]
    AirBurst,
    #[serde(rename = "EWeaponAltFireDisplayType::Shotgun")]
    Shotgun,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponStatsADSStats {
    pub zoom_multiplier: f64,
    pub fire_rate: f64,
    pub run_speed_multiplier: f64,
    pub burst_count: i64,
    pub first_bullet_accuracy: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponStatsAltShotgunStats {
    pub shotgun_pellet_count: i64,
    pub burst_rate: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponStatsAirBurstStats {
    pub shotgun_pellet_count: i64,
    pub burst_distance: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponStatsDamageRange {
    pub range_start_meters: i64,
    pub range_end_meters: i64,
    pub head_damage: f64,
    pub body_damage: f64,
    pub leg_damage: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponShopData {
    pub cost: u64,
    pub category: WeaponShopDataCategory,
    pub shop_order_priority: i64,
    pub category_text: String,
    pub grid_position: Option<WeaponShopDataGridPosition>,
    pub can_be_trashed: bool,
    // pub image: // Currently always null
    pub new_image: String,
    // pub new_image2: // Currently always null
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponShopDataGridPosition {
    pub row: u8,
    pub column: u8,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum WeaponShopDataCategory {
    #[serde(rename = "Heavy Weapons")]
    HeavyWeapons,
    #[serde(rename = "Pistols")]
    Pistols,
    #[serde(rename = "Rifles")]
    Rifles,
    #[serde(rename = "Shotguns")]
    Shotguns,
    #[serde(rename = "SMGs")]
    SMGs,
    #[serde(rename = "Sniper Rifles")]
    SniperRifles,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSkin {
    pub uuid: Uuid,
    pub display_name: String,
    pub theme_uuid: Uuid,
    pub content_tier_uuid: Option<Uuid>,
    pub display_icon: Option<String>,
    pub wallpaper: Option<String>,
    pub asset_path: String,
    pub chromas: Vec<WeaponSkinChroma>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSkinChroma {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub full_render: String,
    pub swatch: Option<String>,
    pub streamed_video: Option<String>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSkinLevels {
    pub uuid: Uuid,
    pub display_name: String,
    pub display_icon: Option<String>,
    pub level_item: Option<WeaponSkinLevelsLevelItem>,
    pub streamed_video: Option<String>,
    pub asset_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum WeaponSkinLevelsLevelItem {
    #[serde(rename = "EEquippableSkinLevelItem::Animation")]
    Animation,
    #[serde(rename = "EEquippableSkinLevelItem::AttackerDefenderSwap")]
    AttackerDefenderSwap,
    #[serde(rename = "EEquippableSkinLevelItem::Finisher")]
    Finisher,
    #[serde(rename = "EEquippableSkinLevelItem::FishAnimation")]
    FishAnimation,
    #[serde(rename = "EEquippableSkinLevelItem::HeartbeatAndMapSensor")]
    HeartbeatAndMapSensor,
    #[serde(rename = "EEquippableSkinLevelItem::InspectAndKill")]
    InspectAndKill,
    #[serde(rename = "EEquippableSkinLevelItem::KillBanner")]
    KillBanner,
    #[serde(rename = "EEquippableSkinLevelItem::KillCounter")]
    KillCounter,
    #[serde(rename = "EEquippableSkinLevelItem::KillEffect")]
    KillEffect,
    #[serde(rename = "EEquippableSkinLevelItem::Randomizer")]
    Randomizer,
    #[serde(rename = "EEquippableSkinLevelItem::SoundEffects")]
    SoundEffects,
    #[serde(rename = "EEquippableSkinLevelItem::TopFrag")]
    TopFrag,
    #[serde(rename = "EEquippableSkinLevelItem::Transformation")]
    Transformation,
    #[serde(rename = "EEquippableSkinLevelItem::VFX")]
    VFX,
    #[serde(rename = "EEquippableSkinLevelItem::Voiceover")]
    Voiceover,
}
