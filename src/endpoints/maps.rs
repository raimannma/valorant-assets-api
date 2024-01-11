use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::map::Map;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of maps from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Maps.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Map>>`.
pub async fn get_maps(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Map>> {
    let mut url = Url::parse("https://valorant-api.com/v1/maps").unwrap();
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

/// This function is used to get a specific map from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `map` - A Uuid that represents the unique identifier of the map.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Map.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Map>`.
pub async fn get_map(
    client: &reqwest::Client,
    map: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Map> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/maps/{}", map)).unwrap();
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
