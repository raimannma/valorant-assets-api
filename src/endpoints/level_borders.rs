use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::level_border::LevelBorder;
use url::Url;
use uuid::Uuid;

pub async fn get_level_borders(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<LevelBorder>> {
    let mut url = Url::parse("https://valorant-api.com/v1/levelborders").unwrap();
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

pub async fn get_level_border(
    client: &reqwest::Client,
    level_border: Uuid,
    language: Option<Language>,
) -> reqwest::Result<LevelBorder> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/levelborders/{}",
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
