use crate::models::api_result::APIResult;
use crate::models::gear::Gear;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of gears from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Gears.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Gear>>`.
pub async fn get_gears(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Gear>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gear").unwrap();
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

/// This function is used to get a specific gear from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `gear` - A Uuid that represents the unique identifier of the gear.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Gear.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Gear>`.
pub async fn get_gear(
    client: &reqwest::Client,
    gear: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Gear> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/gear/{}", gear)).unwrap();
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
