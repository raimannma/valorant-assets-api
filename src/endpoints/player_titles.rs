use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::player_title::PlayerTitle;
use url::Url;
use uuid::Uuid;

pub async fn get_player_titles(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<PlayerTitle>> {
    let mut url = Url::parse("https://valorant-api.com/v1/playertitles").unwrap();
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

pub async fn get_player_title(
    client: &reqwest::Client,
    level_border: Uuid,
    language: Option<Language>,
) -> reqwest::Result<PlayerTitle> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/playertitles/{}",
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