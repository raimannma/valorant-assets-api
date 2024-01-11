use crate::models::api_result::APIResult;
use crate::models::bundle::Bundle;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_bundles(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Bundle>> {
    let mut url = Url::parse("https://valorant-api.com/v1/bundles").unwrap();
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

pub async fn get_bundle(
    client: &reqwest::Client,
    bundle: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Bundle> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/bundles/{}", bundle)).unwrap();
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
