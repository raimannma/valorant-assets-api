use uuid::Uuid;
use valorant_assets_api::competitive_tiers::{get_competitive_tier, get_competitive_tiers};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let competitve_tiers = get_competitive_tiers(&client, language)
        .await
        .expect("Failed to get competitve tiers");
    assert!(!competitve_tiers.is_empty());

    println!(
        "Competitive Tiers: {:?}",
        competitve_tiers
            .iter()
            .map(|x| x.uuid)
            .collect::<Vec<Uuid>>()
    );

    let competitve_tier_uuid = competitve_tiers[0].uuid;
    let competitve_tier = get_competitive_tier(&client, competitve_tier_uuid, language)
        .await
        .expect("Failed to get competitve tier");
    assert_eq!(competitve_tier, competitve_tiers[0]);
    println!("Competitve Tier: {:#?}", competitve_tier);
}
