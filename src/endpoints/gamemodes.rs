use crate::models::api_result::APIResult;
use crate::models::gamemode::{Gamemode, GamemodeEquippable};
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of game modes from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of GameModes.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<GameMode>>`.
pub async fn get_gamemodes(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Gamemode>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gamemodes").unwrap();
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

/// This function is used to get a specific game mode from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `gamemode` - A Uuid that represents the unique identifier of the game mode.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a GameMode.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<GameMode>`.
pub async fn get_gamemode(
    client: &reqwest::Client,
    gamemode: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Gamemode> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/gamemodes/{}",
        gamemode
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

/// This function is used to get a list of equippables for a specific game mode from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of GameModeEquippable.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<GameModeEquippable>>`.
pub async fn get_gamemode_equippables(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<GamemodeEquippable>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gamemodes/equippables").unwrap();
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

/// This function is used to get a specific equippable for a specific game mode from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `equippable` - A Uuid that represents the unique identifier of the equippable.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a GameModeEquippable.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<GameModeEquippable>`.
pub async fn get_gamemode_equippable(
    client: &reqwest::Client,
    equippable: Uuid,
    language: Option<Language>,
) -> reqwest::Result<GamemodeEquippable> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/gamemodes/equippables/{}",
        equippable
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
