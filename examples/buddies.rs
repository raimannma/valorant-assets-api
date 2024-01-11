use static_valorant_api::buddies::{get_buddies, get_buddy, get_buddy_level, get_buddy_levels};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let buddies = get_buddies(&client, language)
        .await
        .expect("Failed to get buddies");
    assert!(!buddies.is_empty());

    println!(
        "Buddies: {:?}",
        buddies
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let buddy_uuid = buddies[0].uuid;
    let buddy = get_buddy(&client, buddy_uuid, language)
        .await
        .expect("Failed to get agent");
    assert_eq!(buddy, buddies[0]);
    println!("Buddy: {:#?}", buddy);

    let buddy_levels = get_buddy_levels(&client, language)
        .await
        .expect("Failed to get buddy levels");
    assert!(!buddy_levels.is_empty());

    println!(
        "Buddy Levels: {:?}",
        buddy_levels
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let buddy_level_uuid = buddy_levels[0].uuid;
    let buddy_level = get_buddy_level(&client, buddy_level_uuid, language)
        .await
        .expect("Failed to get buddy level");
    assert_eq!(buddy_level, buddy_levels[0]);
    println!("Buddy Level: {:#?}", buddy_level);
}
