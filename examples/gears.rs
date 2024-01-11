use static_valorant_api::gears::{get_gear, get_gears};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let gears = get_gears(&client, language)
        .await
        .expect("Failed to get gears");
    assert!(!gears.is_empty());

    println!(
        "Gears: {:?}",
        gears
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let gear_uuid = gears[0].uuid;
    let gear = get_gear(&client, gear_uuid, language)
        .await
        .expect("Failed to get Gear");
    assert_eq!(gear, gears[0]);
    println!("Gear: {:#?}", gear);
}
