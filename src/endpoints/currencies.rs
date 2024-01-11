use crate::models::api_result::APIResult;
use crate::models::currency::Currency;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of currencies from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Currencies.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Currency>>`.
pub async fn get_currencies(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Currency>> {
    let mut url = Url::parse("https://valorant-api.com/v1/currencies").unwrap();
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

/// This function is used to get a specific currency from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `currency` - A Uuid that represents the unique identifier of the currency.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Currency.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Currency>`.
pub async fn get_currency(
    client: &reqwest::Client,
    currency: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Currency> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/currencies/{}",
        currency
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
