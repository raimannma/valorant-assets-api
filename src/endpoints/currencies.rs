use crate::models::api_result::APIResult;
use crate::models::currency::Currency;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_currencies(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Currency>> {
    let mut url = Url::parse("https://valorant-api.com/v1/currencies").unwrap();
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

pub async fn get_currency(
    client: &reqwest::Client,
    currency: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Currency> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/currencies/{}",
        currency
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
