use crate::models::api_result::APIResult;
use crate::models::ceremony::Ceremony;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of ceremonies from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Ceremonies.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Ceremony>>`.
pub async fn get_ceremonies(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Ceremony>> {
    let mut url = Url::parse("https://valorant-api.com/v1/ceremonies").unwrap();
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

/// This function is used to get a specific ceremony from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `ceremony` - A Uuid that represents the unique identifier of the ceremony.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Ceremony.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Ceremony>`.
pub async fn get_ceremony(
    client: &reqwest::Client,
    ceremony: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Ceremony> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/ceremonies/{}",
        ceremony
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
