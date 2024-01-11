use static_valorant_api::contracts::{get_contract, get_contracts};
use static_valorant_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let contracts = get_contracts(&client, language)
        .await
        .expect("Failed to get contracts");
    assert!(!contracts.is_empty());

    println!(
        "Contracts: {:?}",
        contracts
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let contract_uuid = contracts[0].uuid;
    let contract = get_contract(&client, contract_uuid, language)
        .await
        .expect("Failed to get contract");
    assert_eq!(contract, contracts[0]);
    println!("Contract: {:#?}", contract);
}
