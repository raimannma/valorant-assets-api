use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::season::{CompetitiveSeason, Season};
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of seasons from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Seasons.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Season>>`.
pub async fn get_seasons(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Season>> {
    let mut url = Url::parse("https://valorant-api.com/v1/seasons").unwrap();
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

/// This function is used to get a specific season from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `season` - A Uuid that represents the unique identifier of the season.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Season.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Season>`.
pub async fn get_season(
    client: &reqwest::Client,
    season: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Season> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/seasons/{}", season)).unwrap();
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

/// This function is used to get a list of competitive seasons from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of CompetitiveSeasons.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<CompetitiveSeason>>`.
pub async fn get_competitive_seasons(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<CompetitiveSeason>> {
    let mut url = Url::parse("https://valorant-api.com/v1/seasons/competitive").unwrap();
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

/// This function is used to get a specific competitive season from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `season` - A Uuid that represents the unique identifier of the competitive season.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a CompetitiveSeason.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<CompetitiveSeason>`.
pub async fn get_competitive_season(
    client: &reqwest::Client,
    season: Uuid,
    language: Option<Language>,
) -> reqwest::Result<CompetitiveSeason> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/seasons/competitive/{}",
        season
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
