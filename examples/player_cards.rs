use valorant_assets_api::models::language::Language;
use valorant_assets_api::player_cards::{get_player_card, get_player_cards};

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let player_cards = get_player_cards(&client, language)
        .await
        .expect("Failed to get player cards");
    assert!(!player_cards.is_empty());

    println!(
        "Player Cards: {:?}",
        player_cards
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let player_card_uuid = player_cards[0].uuid;
    let player_card = get_player_card(&client, player_card_uuid, language)
        .await
        .expect("Failed to get player card");
    assert_eq!(player_card, player_cards[0]);
    println!("Player Card: {:#?}", player_card);
}
