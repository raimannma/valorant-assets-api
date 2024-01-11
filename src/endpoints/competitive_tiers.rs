use crate::models::api_result::APIResult;
use crate::models::competitive_tier::CompetitiveTiers;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of competitive tiers from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of CompetitiveTiers.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<CompetitiveTiers>>`.
pub async fn get_competitive_tiers(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<CompetitiveTiers>> {
    let mut url = Url::parse("https://valorant-api.com/v1/competitivetiers").unwrap();
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

/// This function is used to get a specific competitive tier from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `competitive_tier` - A Uuid that represents the unique identifier of the competitive tier.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a CompetitiveTiers.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<CompetitiveTiers>`.
pub async fn get_competitive_tier(
    client: &reqwest::Client,
    competitive_tier: Uuid,
    language: Option<Language>,
) -> reqwest::Result<CompetitiveTiers> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/competitivetiers/{}",
        competitive_tier
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
