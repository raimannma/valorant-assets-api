use valorant_assets_api::level_borders::{get_level_border, get_level_borders};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let level_borders = get_level_borders(&client, language)
        .await
        .expect("Failed to get level borders");
    assert!(!level_borders.is_empty());

    println!(
        "Level Borders: {:?}",
        level_borders
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let level_border_uuid = level_borders[0].uuid;
    let level_border = get_level_border(&client, level_border_uuid, language)
        .await
        .expect("Failed to get level border");
    assert_eq!(level_border, level_borders[0]);
    println!("Level Border: {:#?}", level_border);
}
