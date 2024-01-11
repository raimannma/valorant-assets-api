use valorant_assets_api::models::language::Language;
use valorant_assets_api::sprays::{get_spray, get_spray_level, get_spray_levels, get_sprays};

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let sprays = get_sprays(&client, language)
        .await
        .expect("Failed to get sprays");
    assert!(!sprays.is_empty());

    println!(
        "Sprays: {:?}",
        sprays
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let spray_uuid = sprays[0].uuid;
    let spray = get_spray(&client, spray_uuid, language)
        .await
        .expect("Failed to get Spray");
    assert_eq!(spray, sprays[0]);
    println!("Spray: {:#?}", spray);

    let spray_levels = get_spray_levels(&client, language)
        .await
        .expect("Failed to get spray levels");
    assert!(!spray_levels.is_empty());

    println!(
        "Spray Levels: {:?}",
        spray_levels
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let spray_level_uuid = spray_levels[0].uuid;
    let spray_level = get_spray_level(&client, spray_level_uuid, language)
        .await
        .expect("Failed to get Spray Level");
    assert_eq!(spray_level, spray_levels[0]);
    println!("Spray Level: {:#?}", spray_level);
}
