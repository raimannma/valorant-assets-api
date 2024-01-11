use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::theme::Theme;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of themes from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Themes.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Theme>>`.
pub async fn get_themes(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Theme>> {
    let mut url = Url::parse("https://valorant-api.com/v1/themes").unwrap();
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

/// This function is used to get a specific theme from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `theme` - A Uuid that represents the unique identifier of the theme.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Theme.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Theme>`.
pub async fn get_theme(
    client: &reqwest::Client,
    theme: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Theme> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/themes/{}", theme)).unwrap();
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
