use crate::models::api_result::APIResult;
use crate::models::buddy::{Buddy, BuddyLevel};
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of buddies from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Buddies.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Buddy>>`.
pub async fn get_buddies(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Buddy>> {
    let mut url = Url::parse("https://valorant-api.com/v1/buddies").unwrap();
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

/// This function is used to get a specific buddy from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `buddy` - A Uuid that represents the unique identifier of the buddy.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Buddy.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Buddy>`.
pub async fn get_buddy(
    client: &reqwest::Client,
    buddy: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Buddy> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/buddies/{}", buddy)).unwrap();
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

/// This function is used to get a list of buddy levels from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of BuddyLevels.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<BuddyLevel>>`.
pub async fn get_buddy_levels(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<BuddyLevel>> {
    let mut url = Url::parse("https://valorant-api.com/v1/buddies/levels").unwrap();
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

/// This function is used to get a specific buddy level from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `buddy_level` - A Uuid that represents the unique identifier of the buddy level.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a BuddyLevel.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<BuddyLevel>`.
pub async fn get_buddy_level(
    client: &reqwest::Client,
    buddy_level: Uuid,
    language: Option<Language>,
) -> reqwest::Result<BuddyLevel> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/buddies/levels/{}",
        buddy_level
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
