use crate::models::api_result::APIResult;
use crate::models::content_tier::ContentTier;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_content_tiers(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<ContentTier>> {
    let mut url = Url::parse("https://valorant-api.com/v1/contenttiers").unwrap();
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

pub async fn get_content_tier(
    client: &reqwest::Client,
    content_tier: Uuid,
    language: Option<Language>,
) -> reqwest::Result<ContentTier> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/contenttiers/{}",
        content_tier
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
