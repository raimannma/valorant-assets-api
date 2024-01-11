use valorant_assets_api::bundles::{get_bundle, get_bundles};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let bundles = get_bundles(&client, language)
        .await
        .expect("Failed to get bundles");
    assert!(!bundles.is_empty());

    println!(
        "Bundles: {:?}",
        bundles
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let bundle_uuid = bundles[0].uuid;
    let bundle = get_bundle(&client, bundle_uuid, language)
        .await
        .expect("Failed to get bundle");
    assert_eq!(bundle, bundles[0]);
    println!("Bundle: {:#?}", bundle);
}
