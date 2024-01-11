use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::player_title::PlayerTitle;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of player titles from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of PlayerTitles.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<PlayerTitle>>`.
pub async fn get_player_titles(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<PlayerTitle>> {
    let mut url = Url::parse("https://valorant-api.com/v1/playertitles").unwrap();
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

/// This function is used to get a specific player title from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `level_border` - A Uuid that represents the unique identifier of the player title.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a PlayerTitle.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<PlayerTitle>`.
pub async fn get_player_title(
    client: &reqwest::Client,
    level_border: Uuid,
    language: Option<Language>,
) -> reqwest::Result<PlayerTitle> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/playertitles/{}",
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
