use static_valorant_api::models::language::Language;
use static_valorant_api::player_titles::{get_player_title, get_player_titles};

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let player_titles = get_player_titles(&client, language)
        .await
        .expect("Failed to get player Titles");
    assert!(!player_titles.is_empty());

    println!(
        "Player Titles: {:?}",
        player_titles
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<Option<String>>>()
    );

    let player_title_uuid = player_titles[0].uuid;
    let player_title = get_player_title(&client, player_title_uuid, language)
        .await
        .expect("Failed to get player title");
    assert_eq!(player_title, player_titles[0]);
    println!("Player Title: {:#?}", player_title);
}
