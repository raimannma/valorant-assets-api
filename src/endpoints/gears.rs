use crate::models::api_result::APIResult;
use crate::models::gear::Gear;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_gears(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Gear>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gear").unwrap();
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

pub async fn get_gear(
    client: &reqwest::Client,
    gear: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Gear> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/gear/{}", gear)).unwrap();
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
