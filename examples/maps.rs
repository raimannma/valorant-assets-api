use static_valorant_api::maps::{get_map, get_maps};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let maps = get_maps(&client, language)
        .await
        .expect("Failed to get maps");
    assert!(!maps.is_empty());

    println!(
        "Maps: {:?}",
        maps.iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let map_uuid = maps[0].uuid;
    let map = get_map(&client, map_uuid, language)
        .await
        .expect("Failed to get Map");
    assert_eq!(map, maps[0]);
    println!("Map: {:#?}", map);
}
