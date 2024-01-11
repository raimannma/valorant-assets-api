use crate::models::api_result::APIResult;
use crate::models::buddy::{Buddy, BuddyLevel};
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

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
