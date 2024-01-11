use crate::models::api_result::APIResult;
use crate::models::competitive_tier::CompetitiveTiers;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_competitive_tiers(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<CompetitiveTiers>> {
    let mut url = Url::parse("https://valorant-api.com/v1/competitivetiers").unwrap();
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

pub async fn get_competitive_tier(
    client: &reqwest::Client,
    competitive_tier: Uuid,
    language: Option<Language>,
) -> reqwest::Result<CompetitiveTiers> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/competitivetiers/{}",
        competitive_tier
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
