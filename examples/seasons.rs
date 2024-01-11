use static_valorant_api::models::language::Language;
use static_valorant_api::seasons::{
    get_competitive_season, get_competitive_seasons, get_season, get_seasons,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let seasons = get_seasons(&client, language)
        .await
        .expect("Failed to get seasons");
    assert!(!seasons.is_empty());

    println!(
        "Seasons: {:?}",
        seasons
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let season_uuid = seasons[0].uuid;
    let season = get_season(&client, season_uuid, language)
        .await
        .expect("Failed to get season");
    assert_eq!(season, seasons[0]);
    println!("Season: {:#?}", season);

    let competitive_seasons = get_competitive_seasons(&client, language)
        .await
        .expect("Failed to get competitive seasons");
    assert!(!competitive_seasons.is_empty());

    println!(
        "Competitive Seasons: {:?}",
        competitive_seasons
            .iter()
            .map(|x| x.uuid)
            .collect::<Vec<Uuid>>()
    );

    let competitive_season_uuid = competitive_seasons[0].uuid;
    let competitive_season = get_competitive_season(&client, competitive_season_uuid, language)
        .await
        .expect("Failed to get competitive season");
    assert_eq!(competitive_season, competitive_seasons[0]);
    println!("Competitive Season: {:#?}", competitive_season);
}
