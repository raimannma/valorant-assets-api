use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::theme::Theme;
use url::Url;
use uuid::Uuid;

pub async fn get_themes(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Theme>> {
    let mut url = Url::parse("https://valorant-api.com/v1/themes").unwrap();
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

pub async fn get_theme(
    client: &reqwest::Client,
    theme: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Theme> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/themes/{}", theme)).unwrap();
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
