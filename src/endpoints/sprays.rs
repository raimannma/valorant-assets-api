use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::spray::{Spray, SprayLevel};
use url::Url;
use uuid::Uuid;

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
