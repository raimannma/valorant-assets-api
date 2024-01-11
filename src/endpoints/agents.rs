use crate::models::agent::Agent;
use crate::models::api_result::APIResult;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_agents(
    client: &reqwest::Client,
    language: Option<Language>,
    is_playable_character: Option<bool>,
) -> reqwest::Result<Vec<Agent>> {
    let mut url = Url::parse("https://valorant-api.com/v1/agents").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    if let Some(is_playable_character) = is_playable_character {
        url.query_pairs_mut()
            .append_pair("isPlayableCharacter", &format!("{}", is_playable_character));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

pub async fn get_agent(
    client: &reqwest::Client,
    agent: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Agent> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/agents/{}", agent)).unwrap();
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
