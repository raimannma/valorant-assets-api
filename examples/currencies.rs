use valorant_assets_api::currencies::{get_currencies, get_currency};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let currencies = get_currencies(&client, language)
        .await
        .expect("Failed to get currencies");
    assert!(!currencies.is_empty());

    println!(
        "Currencies: {:?}",
        currencies
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let currency_uuid = currencies[0].uuid;
    let currency = get_currency(&client, currency_uuid, language)
        .await
        .expect("Failed to get currency");
    assert_eq!(currency, currencies[0]);
    println!("Currency: {:#?}", currency);
}
