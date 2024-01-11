use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::spray::{Spray, SprayLevel};
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of sprays from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Sprays.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Spray>>`.
pub async fn get_sprays(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Spray>> {
    let mut url = Url::parse("https://valorant-api.com/v1/sprays").unwrap();
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

/// This function is used to get a specific spray from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `spray` - A Uuid that represents the unique identifier of the spray.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Spray.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Spray>`.
pub async fn get_spray(
    client: &reqwest::Client,
    spray: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Spray> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/sprays/{}", spray)).unwrap();
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

/// This function is used to get a list of spray levels from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of SprayLevels.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<SprayLevel>>`.
pub async fn get_spray_levels(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<SprayLevel>> {
    let mut url = Url::parse("https://valorant-api.com/v1/sprays/levels").unwrap();
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

/// This function is used to get a specific spray level from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `spray_level` - A Uuid that represents the unique identifier of the spray level.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a SprayLevel.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<SprayLevel>`.
pub async fn get_spray_level(
    client: &reqwest::Client,
    spray_level: Uuid,
    language: Option<Language>,
) -> reqwest::Result<SprayLevel> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/sprays/levels/{}",
        spray_level
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
