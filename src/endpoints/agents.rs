use crate::models::agent::Agent;
use crate::models::api_result::APIResult;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of agents from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
/// * `is_playable_character` - An Option that may contain a bool. If Some, this bool is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Agents.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Agent>>`.
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

/// This function is used to get a specific agent from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `agent` - A Uuid that represents the unique identifier of the agent.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains an Agent.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Agent>`.
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
