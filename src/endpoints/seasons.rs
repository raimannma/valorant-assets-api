use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::season::{CompetitiveSeason, Season};
use url::Url;
use uuid::Uuid;

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
