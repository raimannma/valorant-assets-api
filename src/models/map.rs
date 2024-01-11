use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub uuid: Uuid,
    pub display_name: String,
    pub narrative_description: Option<String>,
    pub tactical_description: Option<String>,
    pub coordinates: Option<String>,
    pub display_icon: Option<String>,
    pub list_view_icon: String,
    pub list_view_icon_tall: Option<String>,
    pub splash: String,
    pub stylized_background_image: Option<String>,
    pub premier_background_image: Option<String>,
    pub asset_path: String,
    pub map_url: String,
    pub x_multiplier: f64,
    pub y_multiplier: f64,
    pub x_scalar_to_add: f64,
    pub y_scalar_to_add: f64,
    pub callouts: Option<Vec<MapCallout>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapCallout {
    pub region_name: String,
    pub super_region_name: String,
    pub location: Location,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub x: f64,
    pub y: f64,
}
