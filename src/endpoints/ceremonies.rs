use crate::models::api_result::APIResult;
use crate::models::ceremony::Ceremony;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_ceremonies(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Ceremony>> {
    let mut url = Url::parse("https://valorant-api.com/v1/ceremonies").unwrap();
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

pub async fn get_ceremony(
    client: &reqwest::Client,
    ceremony: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Ceremony> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/ceremonies/{}",
        ceremony
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
