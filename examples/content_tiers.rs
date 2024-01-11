use static_valorant_api::content_tiers::{get_content_tier, get_content_tiers};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let content_tiers = get_content_tiers(&client, language)
        .await
        .expect("Failed to get competitve tiers");
    assert!(!content_tiers.is_empty());

    println!(
        "Content Tiers: {:?}",
        content_tiers
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let content_tier_uuid = content_tiers[0].uuid;
    let content_tier = get_content_tier(&client, content_tier_uuid, language)
        .await
        .expect("Failed to get competitve tier");
    assert_eq!(content_tier, content_tiers[0]);
    println!("Content Tier: {:#?}", content_tier);
}
