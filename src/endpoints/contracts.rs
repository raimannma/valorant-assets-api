use crate::models::api_result::APIResult;
use crate::models::contract::Contract;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of contracts from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Contracts.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Contract>>`.
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

/// This function is used to get a specific contract from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `contract` - A Uuid that represents the unique identifier of the contract.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Contract.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Contract>`.
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
