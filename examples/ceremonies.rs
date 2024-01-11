use static_valorant_api::ceremonies::{get_ceremonies, get_ceremony};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let ceremonies = get_ceremonies(&client, language)
        .await
        .expect("Failed to get ceremonies");
    assert!(!ceremonies.is_empty());

    println!(
        "Ceremonies: {:?}",
        ceremonies
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let cerenomy_uuid = ceremonies[0].uuid;
    let ceremony = get_ceremony(&client, cerenomy_uuid, language)
        .await
        .expect("Failed to get ceremony");
    assert_eq!(ceremony, ceremonies[0]);
    println!("Ceremony: {:#?}", ceremony);
}
