use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::level_border::LevelBorder;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of level borders from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of LevelBorders.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<LevelBorder>>`.
pub async fn get_level_borders(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<LevelBorder>> {
    let mut url = Url::parse("https://valorant-api.com/v1/levelborders").unwrap();
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

/// This function is used to get a specific level border from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `level_border` - A Uuid that represents the unique identifier of the level border.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a LevelBorder.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<LevelBorder>`.
pub async fn get_level_border(
    client: &reqwest::Client,
    level_border: Uuid,
    language: Option<Language>,
) -> reqwest::Result<LevelBorder> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/levelborders/{}",
        level_border
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
