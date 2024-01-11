use static_valorant_api::gamemodes::{
    get_gamemode, get_gamemode_equippable, get_gamemode_equippables, get_gamemodes,
};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let gamemodes = get_gamemodes(&client, language)
        .await
        .expect("Failed to get gamemodes");
    assert!(!gamemodes.is_empty());

    println!(
        "Gamemodes: {:?}",
        gamemodes
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let gamemode_uuid = gamemodes[0].uuid;
    let gamemode = get_gamemode(&client, gamemode_uuid, language)
        .await
        .expect("Failed to get gamemode");
    assert_eq!(gamemode, gamemodes[0]);
    println!("Gamemode: {:#?}", gamemode);

    let gamemode_equippables = get_gamemode_equippables(&client, language)
        .await
        .expect("Failed to get gamemode equippables");
    assert!(!gamemode_equippables.is_empty());

    println!(
        "Gamemode Equippables: {:?}",
        gamemode_equippables
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let gamemode_uuid = gamemode_equippables[0].uuid;
    let gamemode_equippable = get_gamemode_equippable(&client, gamemode_uuid, language)
        .await
        .expect("Failed to get gamemode equippable");
    assert_eq!(gamemode_equippable, gamemode_equippables[0]);
    println!("Gamemode Equippable: {:#?}", gamemode_equippable);
}
