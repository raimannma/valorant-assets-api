use crate::models::api_result::APIResult;
use crate::models::content_tier::ContentTier;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of content tiers from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of ContentTiers.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<ContentTier>>`.
pub async fn get_content_tiers(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<ContentTier>> {
    let mut url = Url::parse("https://valorant-api.com/v1/contenttiers").unwrap();
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

/// This function is used to get a specific content tier from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `content_tier` - A Uuid that represents the unique identifier of the content tier.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a ContentTier.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<ContentTier>`.
pub async fn get_content_tier(
    client: &reqwest::Client,
    content_tier: Uuid,
    language: Option<Language>,
) -> reqwest::Result<ContentTier> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/contenttiers/{}",
        content_tier
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
