use valorant_assets_api::events::{get_event, get_events};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let events = get_events(&client, language)
        .await
        .expect("Failed to get events");
    assert!(!events.is_empty());

    println!(
        "Events: {:?}",
        events
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let event_uuid = events[0].uuid;
    let event = get_event(&client, event_uuid, language)
        .await
        .expect("Failed to get Event");
    assert_eq!(event, events[0]);
    println!("Event: {:#?}", event);
}
