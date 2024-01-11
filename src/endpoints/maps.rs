use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::map::Map;
use url::Url;
use uuid::Uuid;

pub async fn get_maps(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Map>> {
    let mut url = Url::parse("https://valorant-api.com/v1/maps").unwrap();
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

pub async fn get_map(
    client: &reqwest::Client,
    map: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Map> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/maps/{}", map)).unwrap();
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
