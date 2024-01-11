use crate::models::api_result::APIResult;
use crate::models::contract::Contract;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_contracts(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Contract>> {
    let mut url = Url::parse("https://valorant-api.com/v1/contracts").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

pub async fn get_contract(
    client: &reqwest::Client,
    contract: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Contract> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/contracts/{}",
        contract
    ))
    .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}
